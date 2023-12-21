use serde::{Serialize, Serializer};

mod file_browser;
mod manager;
mod logging;

pub use file_browser::FileBrowserBindings;
pub use manager::ManagerBindings;
pub use logging::*;

pub struct Null;

impl Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_none()
    }
}
