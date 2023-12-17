use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone)]
pub struct Mod {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AppState {
    ModList,
    FileBrowser,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum CommonPath {
    Home, Documents, Downloads, Desktop
}

impl CommonPath {
    pub fn to_translation_string(&self) -> &'static str {
        match self {
            CommonPath::Home => "page:file_browser:home",
            CommonPath::Documents => "page:file_browser:documents",
            CommonPath::Downloads => "page:file_browser:downloads",
            CommonPath::Desktop => "page:file_browser:desktop",
        }
    }

    pub fn to_svg_path(&self) -> &'static str {
        match self {
            CommonPath::Home => "public/images/home.svg",
            CommonPath::Documents => "public/images/home.svg",
            CommonPath::Downloads => "public/images/home.svg",
            CommonPath::Desktop => "public/images/home.svg",
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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
