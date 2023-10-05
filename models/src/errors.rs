use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AddModError {
    CouldNotOpenModFile,
    InvalidFilePath(String),
    CouldNotReadFile{
        description: String,
    },
    ErrorUnpackingFile(UnpackingFileError),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum UnpackingFileError {
    InvalidFileSignature,
    UnsupportedPackageVersion {
        version: u32,
    },
    InvalidHeader,
    InvalidFileList,
    CouldNotReadPackagedFile,

    MissingMetaData,
    MetaDataInvalidUtf8,
    MetaDataNotXml,
    MetaDataMissingModuleInfo,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum RemoveModError {
    ModWithIndexDoesNotExist(usize),
    ErrorRemovingModFile,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SaveStateError {
    CouldNotCreateOrOpenFile,
    CouldNotSaveToFile
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FileBrowserRedirectError {
    PathDoesNotLeadToDir
}