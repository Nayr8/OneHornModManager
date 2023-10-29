use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use roxmltree::Node;
use models::UnpackingFileError;
use byteorder::{ReadBytesExt, LE};
use serde::{Deserialize, Serialize};
use spin::Mutex;

use models::UnpackingFileError::*;
use crate::{debug, error, trace};


#[derive(Debug)]
#[allow(dead_code)]
pub struct PackagedFileInfo {
    name: String,
    archive_part: u32,
    crc: u32,
    flags: u32,
    offset_in_file: usize,
    size_on_disk: usize,
    uncompressed_size: usize,
}

impl PackagedFileInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn offset_in_file(&self) -> usize {
        self.offset_in_file
    }

    pub fn size_on_disk(&self) -> usize {
        self.size_on_disk
    }

    pub fn uncompressed_size(&self) -> usize {
        self.uncompressed_size
    }

    fn create_string_from_entry(entry: &FileEntry) -> Result<String, UnpackingFileError> {
        let mut name_vec = Vec::new();
        for byte in entry.name {
            if byte == 0 {
                break;
            }
            name_vec.push(byte);
        }
        String::from_utf8(name_vec).map_err(|_| InvalidFileList)
    }
}

impl TryFrom<FileEntry> for Option<PackagedFileInfo> {
    type Error = UnpackingFileError;

    fn try_from(entry: FileEntry) -> Result<Self, Self::Error> {

        let name = PackagedFileInfo::create_string_from_entry(&entry)?;
        if !name.ends_with("meta.lsx") {
            return Ok(None);
        }

        let info: PackagedFileInfo = PackagedFileInfo {
            name,
            archive_part: entry.archive_part as u32,
            crc: 0,
            flags: entry.flags as u32,
            offset_in_file: ((entry.offset_in_file_1 as u64) | ((entry.offset_in_file_2 as u64) << 32)) as usize,
            size_on_disk: entry.size_on_disk as usize,
            uncompressed_size: entry.uncompressed_size as usize,
        };

        Ok(Some(info))
    }
}

pub(crate) struct ModPackage {
    file_info: Vec<PackagedFileInfo>,
    file: Mutex<File>,
}

impl ModPackage {
    const SIGNATURE: u32 = 0x4B50534C;

    pub fn new(mut file: File) -> Result<ModPackage, UnpackingFileError> {
        debug!("Reading meta from package");
        trace!("Checking that file signature is 0x{:X}", Self::SIGNATURE);
        let signature = file.read_u32::<LE>().map_err(|_| {
            error!("Package signature exceeds end of file");
            InvalidFileSignature
        })?;
        if signature != Self::SIGNATURE{
            error!("Signature 0x{signature:X} is invaild");
            return Err(InvalidFileSignature);
        }

        let header = match PackageHeader::read(&mut file) {
            Ok(header) => header,
            Err(error) => {
                error!("Package header exceeds end of file");
                return Err(error);
            }
        };


        if header.version != 18 {
            error!("Unsupported package version: {}", header.version);
            return Err(UnsupportedPackageVersion {
                version: header.version
            })
        }

        Ok(ModPackage {
            file_info: match Self::read_file_list(&mut file, &header) {
                Ok(file_info) => file_info,
                Err(error) => {
                    error!("Could not read file list: {error:?}");
                    return Err(error);
                }
            },
            file: Mutex::new(file),
        })
    }

    fn read_file_list(file: &mut File, header: &PackageHeader) -> Result<Vec<PackagedFileInfo>, UnpackingFileError> {
        debug!("Reading package file list");
        file.seek(SeekFrom::Start(header.file_list_offset as u64)).map_err(|_| InvalidFileList)?;

        let number_of_files = file.read_u32::<LE>().map_err(|_| InvalidFileList)? as usize;
        trace!("{number_of_files} files found in package");
        let compressed_size = file.read_u32::<LE>().map_err(|_| InvalidFileList)? as usize;
        trace!("File list compressed size: {compressed_size}");

        let mut compressed_file_list = vec![0_u8; compressed_size];
        file.read_exact(&mut compressed_file_list).map_err(|_| InvalidFileList)?;

        const FILE_ENTRY_SIZE: usize = 274;
        let file_buffer_size = FILE_ENTRY_SIZE * number_of_files;

        trace!("Decompressing file list");
        let uncompressed_list = match lz4_flex::decompress(compressed_file_list.as_slice(), file_buffer_size) {
            Ok(uncompressed_list) => if uncompressed_list.len() != file_buffer_size {
                uncompressed_list
            } else {
                return Err(InvalidFileList)
            },
            Err(_) => return Err(InvalidFileList),
        };

        let mut file_infos: Vec<PackagedFileInfo> = Vec::with_capacity(number_of_files);
        let mut list_reader = Cursor::new(uncompressed_list);

        trace!("Searching for meta files");
        for _ in 0..number_of_files {
            if let Some(file_entry) = FileEntry::read(&mut list_reader)? {
                if let Some(packaged_file_info) = Option::<PackagedFileInfo>::try_from(file_entry)? {
                    file_infos.push(packaged_file_info);
                }
            }
        }
        match file_infos.len() {
            0 => {
                error!("Could not find a meta file ");
                return Err(MissingMetaData);
            },
            1 => debug!("1 metadata file found: {}", file_infos[0].name),
            meta_file_count => {
                let mut found_meta_files = String::new();
                found_meta_files.push('[');
                for i in 0..meta_file_count - 1 {
                    found_meta_files.push_str(&file_infos[i].name);
                    found_meta_files.push(',');
                    found_meta_files.push(' ');
                }
                found_meta_files.push_str(&file_infos[meta_file_count - 1].name);
                found_meta_files.push(']');
                debug!("{meta_file_count} metadata files found: {}", found_meta_files)
            },
        }

        Ok(file_infos)
    }

    fn read_file(&self, file_info: &PackagedFileInfo) -> Result<Vec<u8>, UnpackingFileError> {
        self.file.lock().seek(SeekFrom::Start(file_info.offset_in_file() as u64))
            .map_err(|_| CouldNotReadPackagedFile)?;

        let mut compressed_file = vec!(0_u8; file_info.size_on_disk());
        self.file.lock().read_exact(&mut compressed_file).map_err(|_| CouldNotReadPackagedFile)?;

        let uncompressed_file = lz4_flex::decompress(compressed_file.as_slice(), file_info.uncompressed_size())
            .map_err(|_| CouldNotReadPackagedFile)?;

        Ok(uncompressed_file)
    }

    pub fn read_package_meta(&self) -> Result<ModMeta, UnpackingFileError> { // TODO add option to specify the file in mods to use for packages with multiple meta files like gustav
        debug!("Searching for first meta.lsx");
        let meta_file_info = match self.file_info.iter().find(|file_info| {
            regex::Regex::new("^Mods/[^/]+/meta.lsx$")
                .expect("Meta data regex is invalid").is_match(file_info.name())
        }) {
            Some(meta_file_info) => meta_file_info,
            None => {
                error!("Could not find a meta file");
                return Err(MissingMetaData)
            }
        };

        let meta_file = String::from_utf8(self.read_file(meta_file_info)?)
            .map_err(|error| {
                error!("Metafile invalid UTF-8: {error:?}");
                MetaDataInvalidUtf8
            })?;

        let meta_xml = roxmltree::Document::parse(&meta_file)
            .map_err(|error| {
                error!("Metafile invalid XML: {error:?}");
                MetaDataNotXml
            })?;

        let module_info = meta_xml.descendants().find(|n| {
            n.attribute("id") == Some("ModuleInfo")
        }).ok_or_else(|| {
            error!("Metadata missing ModuleInfo");
            MetaDataMissingModuleInfo
        })?;

        ModMeta::new(module_info)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModInfoNode {
    pub value_type: String,
    pub value: String,
}

impl ModInfoNode {
    fn new(node: Node) -> Result<ModInfoNode, UnpackingFileError> {
        Ok(ModInfoNode {
            value_type: node.attribute("type").ok_or(MetaDataMissingModuleInfo)?.into(),
            value: node.attribute("value").ok_or(MetaDataMissingModuleInfo)?.into(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModMeta {
    pub name: ModInfoNode,
    pub description: ModInfoNode,
    pub folder: ModInfoNode,
    pub uuid: ModInfoNode,
    pub md5: ModInfoNode,
    pub version64: ModInfoNode,
}

impl ModMeta {
    fn new(module_info: Node) -> Result<ModMeta, UnpackingFileError> {
        let name = match Self::read_property(&module_info, "Name") {
            Ok(name) => name,
            Err(error) => {
                error!("Meta missing mod name");
                return Err(error);
            }
        };
        let folder = match Self::read_property(&module_info, "Folder") {
            Ok(name) => name,
            Err(error) => {
                error!("Meta missing mod name");
                return Err(error);
            }
        };
        let uuid = match Self::read_property(&module_info, "UUID") {
            Ok(name) => name,
            Err(error) => {
                error!("Meta missing mod name");
                return Err(error);
            }
        };
        let md5 = Self::read_property(&module_info, "MD5")
            .unwrap_or(ModInfoNode {
                value_type: String::from("LSString"),
                value: String::new(),
            });
        let version64 = Self::read_property(&module_info, "Version64")
            .unwrap_or(ModInfoNode {
                value_type: String::from("int64"),
                value: String::from("36028797018963968"),
            });
        let description = Self::read_property(&module_info, "Description")
            .unwrap_or(ModInfoNode {
                value_type: String::from("LSString"),
                value: String::new(),
            });

        Ok(ModMeta {
            name,
            description,
            folder,
            uuid,
            md5,
            version64,
        })
    }

    fn read_property(module_info: &Node, id: &str) -> Result<ModInfoNode, UnpackingFileError> {
        ModInfoNode::new(module_info.children()
            .find(|n| n.attribute("id") == Some(id))
            .ok_or(MetaDataMissingModuleInfo)?)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct PackageHeader {
    version: u32,
    file_list_offset: usize,
    file_list_size: usize,
    flags: u8,
    priority: u8,
    md5: [u8; 16],
    num_parts: u16,
}

impl PackageHeader {
    fn read(file: &mut File) -> Result<PackageHeader, UnpackingFileError> {
        debug!("Reading package header");
        const HEADER_START: u64 = 4;
        file.seek(SeekFrom::Start(HEADER_START)).map_err(|_| InvalidHeader)?;

        let version = file.read_u32::<LE>().map_err(|_| InvalidHeader)?;
        let file_list_offset = file.read_u64::<LE>().map_err(|_| InvalidHeader)?;
        let file_list_size = file.read_u32::<LE>().map_err(|_| InvalidHeader)?;
        let flags = file.read_u8().map_err(|_| InvalidHeader)?;
        let priority = file.read_u8().map_err(|_| InvalidHeader)?;
        let mut md5 = [0_u8; 16];
        file.read_exact(&mut md5).map_err(|_| InvalidHeader)?;
        let num_parts = file.read_u16::<LE>().map_err(|_| InvalidHeader)?;

        Ok(PackageHeader {
            version,
            file_list_offset: file_list_offset as usize,
            file_list_size: file_list_size as usize,
            flags,
            priority,
            md5,
            num_parts,
        })
    }
}

pub struct FileEntry {
    name: [u8; 256],
    offset_in_file_1: u32,
    offset_in_file_2: u16,
    archive_part: u8,
    flags: u8,
    size_on_disk: u32,
    uncompressed_size: u32,
}

impl FileEntry {
    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Option<FileEntry>, UnpackingFileError> {
        let mut name = [0_u8; 256];
        cursor.read_exact(&mut name).map_err(|_| InvalidFileList)?;
        if &name[0..4] != b"Mods" {
            cursor.set_position(cursor.position() + 16);
            return Ok(None);
        }

        let offset_in_file_1 = cursor.read_u32::<LE>().map_err(|_| InvalidFileList)?;
        let offset_in_file_2 = cursor.read_u16::<LE>().map_err(|_| InvalidFileList)?;
        let archive_part = cursor.read_u8().map_err(|_| InvalidFileList)?;
        let flags = cursor.read_u8().map_err(|_| InvalidFileList)?;
        let size_on_disk = cursor.read_u32::<LE>().map_err(|_| InvalidFileList)?;
        let uncompressed_size = cursor.read_u32::<LE>().map_err(|_| InvalidFileList)?;

        Ok(Some(FileEntry {
            name,
            offset_in_file_1,
            offset_in_file_2,
            archive_part,
            flags,
            size_on_disk,
            uncompressed_size,
        }))
    }
}