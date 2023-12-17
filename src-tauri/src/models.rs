use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use package_helper::Meta;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModState {
    pub meta: Option<Meta>,
    pub path: PathBuf,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModDetails {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct SelectedNewModInfo {
    pub src_path: PathBuf,
    pub meta: Option<Meta>,
    pub unpacked_data: PathBuf,
}

#[derive(Serialize, PartialEq)]
pub enum CommonPath {
    Home, Documents, Downloads, Desktop
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct FileEntry {
    pub entry_type: EntryType,
    pub path: PathBuf,
    pub file_name: String,
}

#[derive(Serialize, PartialEq, Copy, Clone, Debug)]
pub enum EntryType {
    File,
    Directory,
}
