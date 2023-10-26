use std::ffi::OsString;
use std::path::Path;

pub trait HasExtension {
    fn has_extension(&self, extension: &str) -> bool;
}

impl HasExtension for Path {
    fn has_extension(&self, extension: &str) -> bool {
        if let Some(file_extension) = self.extension() {
            return file_extension == OsString::from(extension);
        }
        false
    }
}