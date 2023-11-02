use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use byteorder::{ReadBytesExt, LE};
use crate::error::PackageReadError;
use crate::file_info::{FileInfoV15, FileInfoV18, PackagedFileInfo};
use crate::models::PackageVersion;
use crate::package::Package;
use crate::package_reader::package_header::{PackageHeaderV15, PackageHeaderV16};

mod package_header;

pub struct PackageReader;

impl PackageReader {
    const SIGNATURE: u32 = 0x4B50534C;

    pub fn read_package(package_path: &Path) -> Result<Package, PackageReadError> {

        let mut file = File::open(package_path).map_err(|_| PackageReadError::CouldNotReadFile)?;

        // Check if DOS:2 DE
        file.seek(SeekFrom::End(-8)).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let header_size = file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let signature = file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        if signature == PackageReader::SIGNATURE {
            file.seek(SeekFrom::End(-(header_size as i64))).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
            return Err(PackageReadError::UnsupportedVersionDOS2DE)
        }

        // Check if DOS:2 or any BG3
        file.seek(SeekFrom::Start(0)).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        let signature = file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;
        if signature == PackageReader::SIGNATURE {
            let version = PackageVersion::from(file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?);

            let package = match version {
                PackageVersion::DivinityOriginalSin => return Err(PackageReadError::UnsupportedVersionDOS),
                PackageVersion::DivinityOriginalSinEnhancedEdition => return Err(PackageReadError::UnsupportedVersionDOSEE),
                PackageVersion::DivinityOriginalSin2 => return Err(PackageReadError::UnsupportedVersionDOS2),
                PackageVersion::DivinityOriginalSin2DefinitiveEdition => return Err(PackageReadError::UnsupportedVersionDOS2DE),
                PackageVersion::BaldursGate3EarlyAccess => Self::read_bg3_ea_package(file, package_path)?,
                PackageVersion::BaldursGate3EarlyAccessPatch4 => Self::read_bg3_ea_patch4_package(file, package_path)?,
                PackageVersion::BaldursGate3 => Self::read_bg3_package(file, package_path)?,
                PackageVersion::Invalid(version) => return Err(PackageReadError::UnsupportedVersion(version)),
            };
            return Ok(package);
        }

        file.seek(SeekFrom::Start(0)).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;

        Err(match file.read_u32::<LE>().map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)? {
            7 => PackageReadError::UnsupportedVersionDOS,
            9 => PackageReadError::UnsupportedVersionDOSEE,
            _ => PackageReadError::NoValidSignatureFound,
        })
    }

    fn read_bg3_ea_package(mut file: File, package_path: &Path) -> Result<Package, PackageReadError> {
        file.seek(SeekFrom::Start(4)).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;

        let header = PackageHeaderV15::read(&mut file)?;

        file.seek(SeekFrom::Start(header.file_list_offset() as u64)).map_err(|_| PackageReadError::FileListOverranEndOfFile)?;

        let files = Self::read_file_list_v15(&mut file)?;

        Ok(Package::new(PackageVersion::BaldursGate3EarlyAccess, header.flags(),
                        header.priority(), files, Self::get_part_paths(package_path, 1)?))
    }

    fn read_bg3_ea_patch4_package(mut file: File, package_path: &Path) -> Result<Package, PackageReadError> {
        file.seek(SeekFrom::Start(4)).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;

        let header = PackageHeaderV16::read(&mut file)?;

        file.seek(SeekFrom::Start(header.file_list_offset() as u64)).map_err(|_| PackageReadError::FileListOverranEndOfFile)?;

        let files = Self::read_file_list_v15(&mut file)?;


        Ok(Package::new(PackageVersion::BaldursGate3EarlyAccessPatch4, header.flags(),
                        header.priority(), files, Self::get_part_paths(package_path, header.num_parts())?))
    }

    fn read_bg3_package(mut file: File, package_path: &Path) -> Result<Package, PackageReadError> {
        file.seek(SeekFrom::Start(4)).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;

        let header = PackageHeaderV16::read(&mut file)?;

        file.seek(SeekFrom::Start(header.file_list_offset() as u64)).map_err(|_| PackageReadError::PackageHeaderOverranEndOfFile)?;

        let files = Self::read_file_list_v18(&mut file)?;

        Ok(Package::new(PackageVersion::BaldursGate3, header.flags(),
                        header.priority(), files, Self::get_part_paths(package_path, header.num_parts())?))
    }

    fn read_file_list_v15(file: &mut File) -> Result<Vec<PackagedFileInfo>, PackageReadError> {
        let number_of_files = file.read_u32::<LE>().map_err(|_| PackageReadError::FileListOverranEndOfFile)? as usize;
        let compressed_size = file.read_u32::<LE>().map_err(|_| PackageReadError::FileListOverranEndOfFile)? as usize;

        let mut compressed_file_list = vec![0_u8; compressed_size];
        file.read_exact(&mut compressed_file_list).map_err(|_| PackageReadError::FileListOverranEndOfFile)?;

        let file_buffer_size = FileInfoV15::size() * number_of_files;
        let mut uncompressed_list = vec![0_u8; file_buffer_size];
        match lz4_flex::decompress_into(&compressed_file_list, &mut uncompressed_list) {
            Ok(size_uncompressed) => if file_buffer_size != size_uncompressed {
                return Err(PackageReadError::CouldNotDecompressFileList);
            },
            Err(_) => return Err(PackageReadError::CouldNotDecompressFileList),
        }

        let mut cursor = Cursor::new(uncompressed_list);
        let mut files = Vec::with_capacity(number_of_files);
        for _ in 0..number_of_files {
            files.push(PackagedFileInfo::from(FileInfoV15::read(&mut cursor)?));
        }
        Ok(files)
    }

    fn read_file_list_v18(file: &mut File) -> Result<Vec<PackagedFileInfo>, PackageReadError> {
        let number_of_files = file.read_u32::<LE>().map_err(|_| PackageReadError::FileListOverranEndOfFile)? as usize;
        let compressed_size = file.read_u32::<LE>().map_err(|_| PackageReadError::FileListOverranEndOfFile)? as usize;

        let mut compressed_file_list = vec![0_u8; compressed_size];
        file.read_exact(&mut compressed_file_list).map_err(|_| PackageReadError::FileListOverranEndOfFile)? ;

        let file_buffer_size = FileInfoV18::size() * number_of_files;
        let mut uncompressed_list = vec![0_u8; file_buffer_size];
        match lz4_flex::decompress_into(&compressed_file_list, &mut uncompressed_list) {
            Ok(size_uncompressed) => if file_buffer_size != size_uncompressed {
                return Err(PackageReadError::CouldNotDecompressFileList);
            },
            Err(_) => return Err(PackageReadError::CouldNotDecompressFileList),
        }

        let mut cursor = Cursor::new(uncompressed_list);
        let mut files = Vec::with_capacity(number_of_files);
        for _ in 0..number_of_files {
            files.push(PackagedFileInfo::from(FileInfoV18::read(&mut cursor)?));
        }
        Ok(files)
    }

    fn get_part_paths(package_path: &Path, part_count: u16) -> Result<Vec<PathBuf>, PackageReadError> {
        let mut paths = vec![package_path.to_owned()];

        for part in 1..part_count {
            paths.push(Self::make_part_filename(package_path, part)?);
        }

        Ok(paths)
    }

    fn make_part_filename(package_path: &Path, part: u16) -> Result<PathBuf, PackageReadError> {
        let base_file_name = package_path.file_stem().ok_or_else(|| PackageReadError::CouldNotReadFile)?.to_string_lossy();
        let extension = package_path.extension().ok_or_else(|| PackageReadError::CouldNotReadFile)?.to_string_lossy();
        let file_name = format!("{base_file_name}_{part}.{extension}");

        let mut path = package_path.to_owned();
        path.set_file_name(&file_name);
        Ok(path)
    }
}