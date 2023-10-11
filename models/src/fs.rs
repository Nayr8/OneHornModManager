use std::path::PathBuf;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FileEntry {
    pub entry_type: EntryType,
    pub path: Rc<PathBuf>,
    pub file_name: Rc<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum EntryType {
    File,
    Directory,
}