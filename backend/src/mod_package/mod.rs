use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::ops::Deref;
use models::UnpackingFileError;
use byteorder::{ReadBytesExt, LE};
use spin::Mutex;

use models::UnpackingFileError::*;
use crate::{debug, error, info, trace};
use crate::mod_package::file::{CompressionMethod, FileEntry, PackagedFileInfo};

pub use meta::ModInfoNode;
pub use crate::mod_package::meta::ModMeta;

mod file;
mod meta;

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
                info!("No meta data found in package. Assuming that it is not required");
            },
            1 => debug!("1 metadata file found: {}", file_infos[0].name()),
            meta_file_count => {
                let mut found_meta_files = String::new();
                found_meta_files.push('[');
                for i in 0..meta_file_count - 1 {
                    found_meta_files.push_str(&file_infos[i].name());
                    found_meta_files.push(',');
                    found_meta_files.push(' ');
                }
                found_meta_files.push_str(&file_infos[meta_file_count - 1].name());
                found_meta_files.push(']');
                debug!("{meta_file_count} metadata files found: {}", found_meta_files)
            },
        }

        Ok(file_infos)
    }

    fn read_file(&self, file_info: &PackagedFileInfo) -> Result<Vec<u8>, UnpackingFileError> {
        self.file.lock().seek(SeekFrom::Start(file_info.offset_in_file() as u64))
            .map_err(|error| {
                error!("File location overran end of package: {error}");
                CouldNotReadPackagedFile
            })?;

        let mut compressed_file = vec!(0_u8; file_info.size_on_disk());
        self.file.lock().read_exact(&mut compressed_file).map_err(|_| CouldNotReadPackagedFile)?;

        let uncompressed_file = match file_info.get_compression_method() {
            CompressionMethod::None => compressed_file,
            CompressionMethod::ZLib => {
                let mut decoder = flate2::read::ZlibDecoder::new(compressed_file.deref());
                let mut uncompressed_file = Vec::new();
                if let Err(error) = decoder.read_to_end(&mut uncompressed_file) {
                    error!("Could not decompress file with zlib: {error}");
                    return Err(CouldNotReadPackagedFile);
                }
                uncompressed_file
            }
            CompressionMethod::LZ4 => lz4_flex::decompress(compressed_file.as_slice(), file_info.uncompressed_size())
                .map_err(|error| {
                    error!("Could not decompress file with lz4: {error}");
                    CouldNotReadPackagedFile
                })?,
            CompressionMethod::Invalid(compression_method) => {
                error!("Invalid compression method: {compression_method}");
                return Err(CouldNotReadPackagedFile);
            }
        };

        Ok(uncompressed_file)
    }

    pub fn read_package_meta(&self) -> Result<Option<ModMeta>, UnpackingFileError> { // TODO add option to specify the file in mods to use for packages with multiple meta files like gustav
        debug!("Searching for first meta.lsx");
        let meta_file_info = match self.file_info.iter().find(|file_info| {
            regex::Regex::new("^Mods/[^/]+/meta.lsx$")
                .expect("Meta data regex is invalid").is_match(file_info.name())
        }) {
            Some(meta_file_info) => meta_file_info,
            None => {
                error!("Could not find a meta file");
                return Ok(None)
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

        Ok(Some(ModMeta::new(module_info)?))
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