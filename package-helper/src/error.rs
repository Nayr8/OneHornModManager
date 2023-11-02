


#[derive(Clone, Copy, Debug)]
pub enum PackageReadError {
    CouldNotReadFile,
    FileListOverranEndOfFile,
    FileInfoOverranEndOfFile,
    CouldNotDecompressFileList,
    PackageHeaderOverranEndOfFile,
    UnsupportedVersionDOS,
    UnsupportedVersionDOSEE,
    UnsupportedVersionDOS2,
    UnsupportedVersionDOS2DE,
    UnsupportedVersion(u32),
    NoValidSignatureFound,
    FileNameNotNullTerminated,
}

#[derive(Clone, Copy, Debug)]
pub enum MetaReadError {
    InvalidArchivePart,
    CannotReadPackage,
    MetaNotValidUtf8,
    MetaNotValidXml,
    MetaDataMissingModuleInfo,
    PackageFileReadError(PackageFileReadError),
}

#[derive(Clone, Copy, Debug)]
pub enum PackageFileReadError {
    UnknownCompressionMethod,
    CouldNotDecompressZLibFile,
    CouldNotDecompressLZ4File,
    FileOffsetOverrunsFile,
}