use std::io::{Cursor, Read};
use byteorder::{LE, ReadBytesExt};
use crate::error::PackageReadError;

#[allow(dead_code)]
pub struct PackagedFileInfo {
    name: String,

    archive_part: u32,
    crc: u32,
    flags: u32,
    offset_in_file: usize,
    size_on_disk: usize,
    uncompressed_size: usize,
    solid: bool,
    solid_offset: usize,
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

    pub fn archive_part(&self) -> usize {
        self.archive_part as usize
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
}

impl From<FileInfoV18> for PackagedFileInfo {
    fn from(value: FileInfoV18) -> Self {
        PackagedFileInfo {
            name: value.name,
            archive_part: 0,
            crc: 0,
            flags: value.flags as u32,
            offset_in_file: value.offset_in_file,
            size_on_disk: value.size_on_disk,
            uncompressed_size: value.uncompressed_size,
            solid: false,
            solid_offset: 0,
        }
    }
}

impl From<FileInfoV15> for PackagedFileInfo {
    fn from(value: FileInfoV15) -> Self {
        PackagedFileInfo {
            name: value.name,
            archive_part: value.archive_part,
            crc: value.crc,
            flags: value.flags,
            offset_in_file: value.offset_in_file,
            size_on_disk: value.size_on_disk,
            uncompressed_size: value.uncompressed_size,
            solid: false,
            solid_offset: 0,
        }
    }
}

#[derive(Debug)]
pub enum CompressionMethod {
    None,
    ZLib,
    LZ4,
    Invalid(u32),
}
#[allow(dead_code)]
pub struct FileInfoV18 {
    name: String,

    offset_in_file: usize,
    archive_part: u8,
    flags: u8,
    size_on_disk: usize,
    uncompressed_size: usize,
}

impl FileInfoV18 {
    pub const fn size() -> usize {
        256 + 4 + 2 + 1 + 1 + 4 + 4
    }

    pub fn read(file_list: &mut Cursor<Vec<u8>>) -> Result<FileInfoV18, PackageReadError> {
        let mut name_bytes = vec![0; 256];
        file_list.read_exact(&mut name_bytes).map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let null_byte_index = name_bytes.iter().position(|byte| *byte == 0)
            .ok_or_else(|| PackageReadError::FileNameNotNullTerminated)?;
        let name = String::from_utf8_lossy(&name_bytes[..null_byte_index]).to_string();


        let offset_in_file_lower = file_list.read_u32::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let offset_in_file_higher = file_list.read_u16::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let offset_in_file = offset_in_file_lower as usize | ((offset_in_file_higher as usize) << 32);

        let archive_part = file_list.read_u8().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let flags = file_list.read_u8().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let size_on_disk = file_list.read_u32::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)? as usize;
        let uncompressed_size = file_list.read_u32::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)? as usize;

        Ok(FileInfoV18 {
            name,
            offset_in_file,
            archive_part,
            flags,
            size_on_disk,
            uncompressed_size,
        })
    }
}

#[allow(dead_code)]
pub struct FileInfoV15 {
    name: String,

    offset_in_file: usize,
    size_on_disk: usize,
    uncompressed_size: usize,
    archive_part: u32,
    flags: u32,
    crc: u32,
    unknown2: u32
}

impl FileInfoV15 {
    pub const fn size() -> usize {
        256 + 8 + 8 + 8 + 4 + 4 + 4 + 4
    }

    pub fn read(file_list: &mut Cursor<Vec<u8>>) -> Result<FileInfoV15, PackageReadError> {
        let mut name_bytes = vec![0; 256];
        file_list.read_exact(&mut name_bytes).map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let null_byte_index = name_bytes.iter().position(|byte| *byte == 0)
            .ok_or_else(|| PackageReadError::FileNameNotNullTerminated)?;
        let name = String::from_utf8_lossy(&name_bytes[..null_byte_index]).to_string();


        let offset_in_file = file_list.read_u64::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)? as usize;
        let size_on_disk = file_list.read_u64::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)? as usize;
        let uncompressed_size = file_list.read_u64::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)? as usize;
        let archive_part = file_list.read_u32::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let flags = file_list.read_u32::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let crc = file_list.read_u32::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;
        let unknown2 = file_list.read_u32::<LE>().map_err(|_| PackageReadError::FileInfoOverranEndOfFile)?;

        Ok(FileInfoV15 {
            name,
            offset_in_file,
            size_on_disk,
            uncompressed_size,
            archive_part,
            flags,
            crc,
            unknown2,
        })
    }
}