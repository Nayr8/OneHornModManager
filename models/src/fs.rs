use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FileEntry {
    pub entry_type: EntryType,
    pub path: PathBuf,
    pub file_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum EntryType {
    File,
    Directory,
}