use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use crate::error::{MetaReadError, PackageFileReadError};
use crate::file_info::{CompressionMethod, PackagedFileInfo};
use crate::meta::Meta;
use crate::models::PackageVersion;

#[allow(dead_code)]
pub struct Package {
    version: PackageVersion,
    priority: u8,
    flags: u8,

    files: Vec<PackagedFileInfo>,
    package_files: Vec<PathBuf>
}

impl Package {
    pub fn new(version: PackageVersion, priority: u8, flags: u8, files: Vec<PackagedFileInfo>,
               package_files: Vec<PathBuf>) -> Package {
        Package {
            version,
            priority,
            flags,
            files,
            package_files,
        }
    }

    fn read_file(file: &mut File, file_info: &PackagedFileInfo) -> Result<Vec<u8>, PackageFileReadError> {
        file.seek(SeekFrom::Start(file_info.offset_in_file() as u64)).map_err(|_| PackageFileReadError::FileOffsetOverrunsFile)?;
        let mut buffer = vec![0; file_info.size_on_disk()];
        file.read_exact(buffer.as_mut_slice()).map_err(|_| PackageFileReadError::FileOffsetOverrunsFile)?;

        return match file_info.get_compression_method() {
            CompressionMethod::None => Ok(buffer),
            CompressionMethod::ZLib => {
                let mut decoder = flate2::read::ZlibDecoder::new(buffer.as_slice());
                let mut uncompressed_file = vec![0; file_info.uncompressed_size()];
                decoder.read_to_end(&mut uncompressed_file).map_err(|_| PackageFileReadError::CouldNotDecompressZLibFile)?;
                Ok(uncompressed_file)
            }
            CompressionMethod::LZ4 => {
                let uncompressed_file = lz4_flex::decompress(buffer.as_slice(), file_info.uncompressed_size())
                    .map_err(|_| PackageFileReadError::CouldNotDecompressLZ4File)?;
                Ok(uncompressed_file)
            }
            CompressionMethod::Invalid(_) => Err(PackageFileReadError::UnknownCompressionMethod)
        }
    }

    pub fn get_meta(&self) -> Result<Vec<Meta>, MetaReadError> {
        let mut metas = Vec::new();

        let mut open_files: Vec<Option<File>> = Vec::with_capacity(self.files.len());
        for _ in 0..self.files.len() {
            open_files.push(None);
        }

        let meta_file_infos = self.files.iter().filter(|file| {
            file.name().ends_with("/meta.lsx") && file.name().starts_with("Mods/")
        });

        for file_info in meta_file_infos {
            let file = {
                match open_files.get_mut(file_info.archive_part()).ok_or_else(|| MetaReadError::InvalidArchivePart)? {
                    Some(file) => file,
                    None => {
                        let path = self.package_files.get(file_info.archive_part())
                            .ok_or_else(|| MetaReadError::InvalidArchivePart)?;

                        let file = File::open(path).map_err(|_| MetaReadError::CannotReadPackage)?;
                        *open_files.get_mut(file_info.archive_part())
                            .ok_or_else(|| MetaReadError::InvalidArchivePart)? = Some(file);

                        open_files.get_mut(file_info.archive_part())
                            .ok_or_else(|| MetaReadError::InvalidArchivePart)?
                            .as_mut().ok_or_else(|| MetaReadError::CannotReadPackage)?
                    }
                }
            };

            let file_contents = String::from_utf8(Self::read_file(file, file_info)
                .map_err(|error| MetaReadError::PackageFileReadError(error))?)
                .map_err(|_| MetaReadError::MetaNotValidUtf8)?;

            let xml = roxmltree::Document::parse(&file_contents)
                .map_err(|_| MetaReadError::MetaNotValidXml)?;

            metas.push(Meta::try_from(xml)?);
        }

        Ok(metas)
    }
}