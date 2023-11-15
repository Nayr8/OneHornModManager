use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FileEntry {
    pub entry_type: EntryType,
    pub path: Arc<PathBuf>,
    pub file_name: Arc<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum EntryType {
    File,
    Directory,
}