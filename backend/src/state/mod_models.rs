use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use package_helper::Meta;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModState {
    pub meta: Option<Meta>,
    pub path: PathBuf,
    pub enabled: bool,
}

impl From<SelectedNewModInfo> for ModState {
    fn from(value: SelectedNewModInfo) -> Self {
        ModState {
            meta: value.meta,
            path: value.unpacked_data.clone(),
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SelectedNewModInfo {
    src_path: PathBuf,
    pub meta: Option<Meta>,
    pub unpacked_data: PathBuf,
}

impl SelectedNewModInfo {
    pub fn new(src_path: PathBuf, meta: Option<Meta>, unpacked_data: PathBuf) -> SelectedNewModInfo {
        SelectedNewModInfo {
            src_path,
            meta,
            unpacked_data,
        }
    }

    pub fn src_path(&self) -> &Path {
        &self.src_path
    }

    pub fn meta(&self) -> Option<&Meta> {
        self.meta.as_ref()
    }
}