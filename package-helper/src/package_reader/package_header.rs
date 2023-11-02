use std::fs::File;
use std::io::Read;
use byteorder::{LE, ReadBytesExt};
use crate::error::PackageReadError;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PackageHeaderV15 {
    version: u32,
    file_list_offset: u64,
    file_list_size: u32,
    flags: u8,
    priority: u8,
    md5: [u8; 16],
}

impl PackageHeaderV15 {
    pub fn read(file: &mut File) -> Result<PackageHeaderV15, PackageReadError> {
        let version = file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let file_list_offset = file.read_u64::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let file_list_size = file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let flags = file.read_u8().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let priority = file.read_u8().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let mut md5 = [0; 16];
        file.read_exact(&mut md5).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;

        Ok(PackageHeaderV15 {
            version,
            file_list_offset,
            file_list_size,
            flags,
            priority,
            md5,
        })
    }

    pub fn file_list_offset(&self) -> usize {
        self.file_list_offset as usize
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PackageHeaderV16 {
    version: u32,
    file_list_offset: u64,
    file_list_size: u32,
    flags: u8,
    priority: u8,
    md5: [u8; 16],
    num_parts: u16,
}

impl PackageHeaderV16 {
    pub fn read(file: &mut File) -> Result<PackageHeaderV16, PackageReadError> {
        let version = file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let file_list_offset = file.read_u64::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let file_list_size = file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let flags = file.read_u8().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let priority = file.read_u8().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let mut md5 = [0; 16];
        file.read_exact(&mut md5).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let num_parts = file.read_u16::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;

        Ok(PackageHeaderV16 {
            version,
            file_list_offset,
            file_list_size,
            flags,
            priority,
            md5,
            num_parts,
        })
    }

    pub fn file_list_offset(&self) -> usize {
        self.file_list_offset as usize
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn num_parts(&self) -> u16 {
        self.num_parts
    }
}