use std::io::{Cursor, Read};
use byteorder::{LE, ReadBytesExt};
use models::UnpackingFileError;
use models::UnpackingFileError::InvalidFileList;

#[derive(Debug)]
pub enum CompressionMethod {
    None,
    ZLib,
    LZ4,
    Invalid(u32),
}

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

    pub(crate) fn get_compression_method(&self) -> CompressionMethod {
        let compression_method = self.flags & 0xF;
        match compression_method {
            0 => CompressionMethod::None,
            1 => CompressionMethod::ZLib,
            2 => CompressionMethod::LZ4,
            _ => CompressionMethod::Invalid(compression_method),
        }
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