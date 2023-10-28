use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use spin::{Mutex, MutexGuard};
use models::{Mod, MMResult, ModDetailsError};
use crate::{error, info, debug, trace};
use crate::mod_package::{ModInfoNode, ModMeta, ModPackage};
use crate::extensions::HasExtension;
use crate::mod_settings_builder::ModSettingsBuilder;

static STATE: Mutex<State> = Mutex::new(State::new());


#[tauri::command(rename_all = "snake_case")]
pub fn get_mods() -> Vec<Mod> {
    let state = State::get();

    let mut mods = Vec::with_capacity(state.mods.len());

    for mod_state in &state.mods {
        mods.push(Mod {
            name: mod_state.meta.name.value.clone(),
            description: mod_state.meta.description.value.clone(),
        });
    }

    mods
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_current_mod() {
    let mut state = State::get();

    let (path, meta) = match state.selected_new_mod_meta.take() {
        Some(meta) => meta,
        None => {
            error!("No mod meta cached");
            return; // TODO return and handle error
        }
    };

    state.mods.push(ModState {
        meta,
        path: path.to_string_lossy().to_string(),
        enabled: true,
    });
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_mod(index: usize) {
    let mut state = State::get();
    state.mods.remove(index);
}

#[tauri::command(rename_all = "snake_case")]
pub fn save() {
    let state = State::get();

    let state_string = match serde_json::to_string::<State>(&state) {
        Ok(state_string) => state_string,
        Err(error) => {
            error!("Could not serialize state: {error:?}");
            return; // TODO return and handle error
        }
    };

    let mut file = match OpenOptions::new().write(true).create(true).truncate(true).open("state.json") {
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
}

#[tauri::command(rename_all = "snake_case")]
pub fn apply() {
    let mut state = State::get();

    info!("Creating symlinks to mod pak files");
    let mut mods_folder_path = PathBuf::from(&state.bg3_appdata);
    mods_folder_path.push("Mods/");

    debug!("mods_folder_path: {:?}", mods_folder_path);

    for entry in std::fs::read_dir(&mods_folder_path).unwrap() {
        if entry.is_err() { continue }
        let entry = entry.unwrap();

        if let Ok(file_type) = entry.file_type() {
            if file_type.is_symlink() {
                symlink::remove_symlink_auto(entry.path()).unwrap();
            }
        }
    }

    for mod_state in &state.mods {
        if !mod_state.enabled { continue }

        let mut path = mods_folder_path.clone();
        let src_path = PathBuf::from(&mod_state.path);
        path.push(src_path.file_name().expect("Mod file not a file"));

        if let Err(error) = symlink::symlink_file(&mod_state.path, path) {
            error!("Could not apply mod '{}': {error}", mod_state.path);
            return;
        }
    }

    info!("Writing mod settings");
    let mut mod_settings_path = PathBuf::from(&state.bg3_appdata);
    mod_settings_path.push("PlayerProfiles/Public/modsettings.lsx");
    debug!("mod_settings_path: {}", mod_settings_path.to_string_lossy());

    let mod_settings = state.build_mod_settings();

    info!("Mod settings: {mod_settings}");

    let mut mod_settings_file = match OpenOptions::new()
        .write(true).create(true).truncate(true)
        .open(&mod_settings_path) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not apply mod_settings: {error:?}");
            return; // TODO return and handle error
        }
    };
    if let Err(error) = mod_settings_file.write_all(mod_settings.as_bytes()) {
        error!("Could not write mod_settings: {error:?}");
        return; // TODO return and handle error
    }
    info!("modsetting.lsx written")
}

#[derive(Serialize, Deserialize)]
pub struct ModState {
    pub meta: ModMeta,
    pub path: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct State {
    #[serde(skip_serializing)]
    selected_new_mod_meta: Option<(PathBuf, ModMeta)>,
    mods: Vec<ModState>,
    gustav_dev_mod_meta: Option<ModMeta>,
    pub(crate) bg3_appdata: String,
}

impl State {
    pub fn get() -> MutexGuard<'static, State> {
        STATE.lock()
    }

    const fn new() -> State {
        State {
            selected_new_mod_meta: None,
            mods: Vec::new(),
            gustav_dev_mod_meta: None,
            bg3_appdata: String::new(),
        }
    }

    fn build_mod_settings(&mut self) -> String {
        let gustav_dev_meta = match self.gustav_dev_mod_meta.as_ref() {
            Some(gustav_dev_meta) => gustav_dev_meta,
            None => {
                self.gustav_dev_mod_meta = Some(ModMeta {
                    name: ModInfoNode {
                        value_type: "LSString".to_string(),
                        value: "GustavDev".to_string(),
                    },
                    description: ModInfoNode {
                        value_type: "".to_string(),
                        value: "".to_string(),
                    },
                    folder: ModInfoNode {
                        value_type: "LSString".to_string(),
                        value: "GustavDev".to_string(),
                    },
                    uuid: ModInfoNode {
                        value_type: "FixedString".to_string(),
                        value: "28ac9ce2-2aba-8cda-b3b5-6e922f71b6b8".to_string(),
                    },
                    md5: ModInfoNode {
                        value_type: "LSString".to_string(),
                        value: "".to_string(),
                    },
                    version64: ModInfoNode {
                        value_type: "int64".to_string(),
                        value: "36028797018963968".to_string(),
                    },
                });
                self.gustav_dev_mod_meta.as_ref().unwrap()
            },
        };

        let xml = ModSettingsBuilder::build(&self.mods, gustav_dev_meta);

        let mut a = Vec::new();
        xml.generate(&mut a).unwrap();

        String::from_utf8(a).unwrap()
    }

    pub fn load() {
        info!("Attempting to load state");
        let mut state_file = match File::open("state.json") {
            Ok(state_file) => state_file,
            Err(e) => match e.kind(){
                ErrorKind::NotFound => {
                    info!("No 'state.json' file found");
                    return;
                }
                _ => {
                    error!("Could not open 'state.json' attempting to remove possibly corrupted file: {e}");
                    if let Err(e) = std::fs::remove_file("state.json") {
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
                    info!("Removed currupted 'state.json' successfully");
                }
                return;
            },
        };

        let mut state = STATE.lock();
        state.mods = state_data.mods;
        state.gustav_dev_mod_meta = state_data.gustav_dev_mod_meta;
        info!("State loaded successfully")
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_mod_details(file_path: PathBuf) -> MMResult<Mod, ModDetailsError> {
    info!("Fetching mod details");

    if !file_path.has_extension("pak") {
        error!("File {file_path:?} is not a pak file");
        return MMResult::Err(ModDetailsError::FilePathDoesNotLeadToValidFile)
    }

    trace!("Checking cache for the meta data for this package");
    let mut state = State::get();
    if let Some((path, meta)) = state.selected_new_mod_meta.as_ref() {
        if *path == file_path {
            debug!("Retrieved meta from cache");
            let details = Mod {
                name: meta.name.value.clone(),
                description: meta.description.value.clone(),
            };
            info!("Returning mod details {{name: {}, description: {}}}", details.name, details.description);
            return MMResult::Ok(details)
        }
    }

    let meta = match get_mod_meta(&file_path) {
        Ok(meta) => meta,
        Err(error) => return MMResult::Err(error)
    };

    let details = Mod {
        name: meta.name.value.clone(),
        description: meta.description.value.clone(),
    };

    state.selected_new_mod_meta = Some((file_path, meta));

    info!("Returning mod details {{name: {}, description: {}}}", details.name, details.description);
    MMResult::Ok(details)
}

fn get_mod_meta(file_path: &Path) -> Result<ModMeta, ModDetailsError> {
    trace!("Attempting to retrieve meta data from path: {file_path:?}");
    let file = File::open(file_path).map_err(|error| {
        error!("Not a file: {error:?}");
        ModDetailsError::FilePathDoesNotLeadToValidFile
    })?;
    trace!("Successfully opened file: {file_path:?}");

    ModPackage::new(file).map_err(|error| {
        error!("Cannot unpack file: {:?}", error);
        ModDetailsError::CannotUnpackPackageFile
    })?.read_package_meta().map_err(|error| {
        error!("Cannot read package meta: {:?}", error);
        ModDetailsError::CannotReadPackageMeta
    })
}
