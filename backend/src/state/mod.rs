use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use spin::{Mutex, MutexGuard};
use models::{Mod, ModDetailsError};
use package_helper::{Meta, PackageReader};
use crate::{error, info};
use crate::mod_settings_builder::ModSettingsBuilder;
use crate::state::mod_models::{ModState, SelectedNewModInfo};

pub mod commands;
pub mod mod_models;

static STATE: Mutex<State> = Mutex::new(State::new());


#[derive(Serialize, Deserialize)]
pub(crate) struct State {
    #[serde(skip)]
    pub(crate) bg3_appdata: String,

    selected_new_mod_info: Option<SelectedNewModInfo>,

    mods: Vec<ModState>,
    gustav_dev_mod_meta: Option<Meta>,
}

impl State {
    pub fn get() -> MutexGuard<'static, State> {
        STATE.lock()
    }

    const fn new() -> State {
        State {
            selected_new_mod_info: None,
            mods: Vec::new(),
            gustav_dev_mod_meta: None,
            bg3_appdata: String::new(),
        }
    }

    fn build_mod_settings(&mut self) -> String {
        let gustav_dev_meta = match self.gustav_dev_mod_meta.as_ref() {
            Some(gustav_dev_meta) => gustav_dev_meta,
            None => {
                self.gustav_dev_mod_meta = Some(Meta::gustav_dev());
                self.gustav_dev_mod_meta.as_ref().unwrap()
            },
        };

        let xml = ModSettingsBuilder::build(&self.mods, gustav_dev_meta);

        let mut a = Vec::new();
        xml.generate(&mut a).unwrap();

        String::from_utf8(a).unwrap()
    }

    fn get_data_dir() -> PathBuf {
        match dirs::data_local_dir() {
            Some(mut dir) => {
                dir.push("Nayr8'sBG3ModManager");
                dir
            },
            None => {
                panic!("Could not get local data directory")
            }
        }
    }

    fn get_mod_store_dir() -> PathBuf {
        let mut dir = State::get_data_dir();
        dir.push("Mods");
        dir
    }

    pub fn save() {
        info!("Saving...");
        let state = State::get();

        let state_string = match serde_json::to_string::<State>(&state) {
            Ok(state_string) => state_string,
            Err(error) => {
                error!("Could not serialize state: {error:?}");
                return; // TODO return and handle error
            }
        };

        let data_dir = State::get_data_dir();
        let mut file = match OpenOptions::new().write(true).create(true).truncate(true).open(data_dir.join("state.json")) {
            Ok(file) => file,
            Err(error) => {
                error!("Could not save file: {error:?}");
                return; // TODO return and handle error
            }
        };

        if let Err(error) = file.write_all(state_string.as_bytes()) {
            error!("Could not save file: {error:?}");
            return; // TODO return and handle error
        }
        info!("Saved successfully");
    }

    pub fn load() {
        info!("Attempting to load state");
        let data_directory = Self::get_data_dir();
        std::fs::create_dir_all(&data_directory).expect("Could not create data directory");
        let mut state_file = match File::open(data_directory.join("state.json")) {
            Ok(state_file) => state_file,
            Err(e) => match e.kind(){
                ErrorKind::NotFound => {
                    info!("No 'state.json' file found");
                    return;
                }
                _ => {
                    error!("Could not open 'state.json' attempting to remove possibly corrupted file: {e}");
                    if let Err(e) = std::fs::remove_file(data_directory.join("state.json")) {
                        error!("Could not delete 'state.json' saving state may not be possible: {e}");
                    }
                    return;
                }
            },
        };

        let mut state_string = String::new();
        if let Err(e) = state_file.read_to_string(&mut state_string) {
            error!("Could not read 'state.json' attempting to remove corrupted file: {e}");
            if let Err(e) = std::fs::remove_file("state.json") {
                error!("Could not delete 'state.json' saving state may not be possible: {e}")
            }
        }

        let state_data = match serde_json::from_str::<State>(&state_string) {
            Ok(state) => state,
            Err(e) => {
                error!("'state.json' is not valid JSON attempting to remove corrupted file: {e}");
                error!("{state_string}");
                if let Err(e) = std::fs::remove_file("state.json") {
                    error!("Could not delete 'state.json' saving state may not be possible: {e}");
                } else {
                    info!("Removed corrupted 'state.json' successfully");
                }
                return;
            },
        };

        let mut state = STATE.lock();
        state.mods = state_data.mods;
        state.gustav_dev_mod_meta = state_data.gustav_dev_mod_meta;
        info!("State loaded successfully")
    }

    #[cfg(target_os = "linux")]
    pub fn find_bg3_app_data(&mut self) {
        let mut steam_dir = steamlocate::SteamDir::locate().unwrap();
        let bg3_steam_app = steam_dir.app(&1086940).unwrap();
        self.bg3_appdata = bg3_steam_app.path
            .join("../../compatdata/1086940/pfx/drive_c/users/steamuser/AppData/Local/Larian Studios/Baldur's Gate 3/").to_string_lossy().to_string();
    }

    #[cfg(target_os = "windows")]
    pub fn find_bg3_app_data(&mut self) {
        let appdata_local = dirs::config_local_dir().unwrap();
        self.bg3_appdata = appdata_local.join("Larian Studios/Baldur's Gate 3/")
    }

    #[cfg(target_os = "macos")]
    pub fn find_bg3_app_data(&mut self) {
        let documents = dirs::document_dir();
        self.bg3_appdata = documents.join("Larian Studios/Baldur's Gate 3/")
    }

    pub fn get_mod_details(meta: Option<&Meta>, file_path: &Path, enabled: bool) -> Mod {
        if let Some(meta) = meta { Mod {
            name: meta.name().value().to_string(),
            description: meta.description().to_string(),
            version: meta.version().to_string(),
            enabled,
        } } else {
            let name = file_path.file_name().unwrap()
                .to_string_lossy()
                .trim_end_matches(".pak")
                .to_string();
            Mod {
                name,
                description: String::new(),
                version: String::new(),
                enabled,
            }
        }
    }
}

fn get_mod_meta(dir_path: &Path) -> Result<Vec<Meta>, ModDetailsError> {
    let file_path = find_pak_path(dir_path).ok_or_else(|| {
        error!("Cannot find package file");
        ModDetailsError::CannotFindPackageFile
    })?;

    let package = PackageReader::read_package(&file_path).map_err(|error| {
        error!("Cannot reading package: {error:?}");
        ModDetailsError::CannotUnpackPackageFile
    })?;

    package.get_meta().map_err(|error| {
        error!("Cannot read package meta: {error:?}");
        ModDetailsError::CannotReadPackageMeta
    })
}

fn find_pak_path(dir_path: &Path) -> Option<PathBuf> {
    let dir = std::fs::read_dir(dir_path).ok()?;
    for entry in dir {
        let Ok(entry) = entry else { continue };

        if entry.file_name().to_string_lossy().ends_with(".pak") {
            info!("Found package: {}", entry.file_name().to_string_lossy());
            return Some(dir_path.join(entry.file_name()));
        }
    }
    None
}
