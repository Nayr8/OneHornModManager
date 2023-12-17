use serde::{Serialize, Serializer};

mod file_browser;

pub use file_browser::FileBrowserBindings;

pub struct Null;

impl Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_none()
    }
}