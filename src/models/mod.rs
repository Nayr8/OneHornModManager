use std::ffi::OsStr;
use std::path::PathBuf;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Clone)]
pub struct Mod {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AppState {
    ModList,
    FileBrowser,
    Profiles,
    Settings,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BrowserState {
    FileBrowser,
    AddMod,
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
            CommonPath::Documents => "public/images/documents.svg",
            CommonPath::Downloads => "public/images/downloads.svg",
            CommonPath::Desktop => "public/images/desktop.svg",
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

impl EntryType {
    pub fn to_svg_path(&self, extension: Option<&OsStr>) -> &'static str {
        let extension = extension.map(OsStr::to_string_lossy);
        let extension: Option<&str> = extension.as_ref().map(|e| e.as_ref());
        match (self, extension) {
            (EntryType::Directory, _) => "public/images/folder.svg",
            (EntryType::File, Some("zip")) => "public/images/file.svg",
            (EntryType::File, Some("pak")) => "public/images/pak.svg",
            (EntryType::File, Some(ext)) => {
                error!("Cannot get icon for file with extension {ext}");
                "public/images/error.svg"
            },
            (EntryType::File, None) => {
                error!("Cannot get icon for file with no extension");
                "public/images/error.svg"
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModDetails {
    pub name: String,
    pub description: String,
}
