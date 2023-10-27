use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use spin::{Mutex, MutexGuard};
use models::{Mod, MMResult, ModDetailsError};
use crate::{error, info, debug, trace};
use crate::mod_package::{ModMeta, ModPackage};
use crate::extensions::HasExtension;

static STATE: Mutex<State> = Mutex::new(State::new());


#[tauri::command(rename_all = "snake_case")]
pub fn get_mods() -> Vec<Mod> {
    let state = State::get();

    let mut mods = Vec::with_capacity(state.mods.len());

    for (meta, _file_name) in &state.mods {
        mods.push(Mod {
            name: meta.name.value.clone(),
            description: meta.description.value.clone(),
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

    state.mods.push((meta, path.to_string_lossy().to_string()));
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_mod(index: usize) {
    let mut state = State::get();
    state.mods.remove(index);
}

const STEAM_APPS: &'static str = "/home/ryan/.steam/steam/steamapps";
// compatdata/1086940/pfx/drive_c/users/steamuser/AppData/Local/Larian Studios/Baldur's Gate 3

#[derive(Serialize, Deserialize)]
pub(crate) struct State {
    selected_new_mod_meta: Option<(PathBuf, ModMeta)>,
    mods: Vec<(ModMeta, String)>,
    gustav_dev_mod_meta: Option<ModMeta>,
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
        }
    }
/*
    fn add_mod(&mut self, path: &str) -> Result<Vec<Mod>, AddModError> {
        let package_file = File::open(path)
            .map_err(|_| AddModError::CouldNotOpenModFile)?;

        let package = ModPackage::new(package_file)
            .map_err(|e| AddModError::ErrorUnpackingFile(e))?;

        let meta = package.read_package_meta()
            .map_err(|e| AddModError::ErrorUnpackingFile(e))?;

        drop(package);

        let file_name = move_mod_file(path)?;

        self.mods.push((meta, file_name));

        Ok(self.get_mods())
    }

    pub fn remove_mod(&mut self, index: usize) -> Result<String, RemoveModError> {
        if index >= self.mods.len() { return Err(RemoveModError::ModWithIndexDoesNotExist(index)) }
        Ok(self.mods.remove(index).1)
    }

    pub fn get_mods(&self) -> Vec<Mod> {
        self.mods.iter().map(|(mod_meta, _file_name)| {
            Mod {
                name: mod_meta.name.value.clone(),
                description: mod_meta.description.value.clone(),
            }
        }).collect()
    }

    pub fn build_mod_settings(&mut self) -> String {
        let gustav_dev_meta = match self.gustav_dev_mod_meta.as_ref() {
            Some(gustav_dev_meta) => gustav_dev_meta,
            None => {
                self.load_gustav_dev_meta().expect("TODO: panic message"); // TODO handle error
                self.gustav_dev_mod_meta.as_ref().unwrap()
            },
        };

        let xml = ModSettingsBuilder::build(&self.mods, gustav_dev_meta);

        let mut a = Vec::new();
        xml.generate(&mut a).unwrap();

        String::from_utf8(a).unwrap()
    }

    fn load_gustav_dev_meta(&mut self) -> Result<(), AddModError> {
        const GUSTAV_PAK: &str = "common/Baldurs Gate 3/Data/Gustav.pak";
        let mut gustav_path = PathBuf::from(STEAM_APPS);
        gustav_path.push(GUSTAV_PAK);

        let package_file = File::open(gustav_path)
            .map_err(|_| AddModError::CouldNotOpenModFile)?;

        let package = ModPackage::new(package_file)
            .map_err(|e| AddModError::ErrorUnpackingFile(e))?;

        let meta = package.read_package_meta()
            .map_err(|e| AddModError::ErrorUnpackingFile(e))?;

        self.gustav_dev_mod_meta = Some(meta);
        Ok(())
    }

    fn save(&mut self) -> Result<(), SaveStateError> {
        info!("Saving state");
        let state_json = serde_json::to_string(State::get().deref()).expect("TODO");

        let mut state_file = match OpenOptions::new().create(true).write(true).truncate(true).open("state.json") {
            Ok(state_file) => state_file,
            Err(e) => {
                error!("Could not create save file: {e}");
                return Err(SaveStateError::CouldNotCreateOrOpenFile);
            }
        };

        if let Err(e) = state_file.write_all(state_json.as_bytes()) {
            error!("Could not save state: {e}");
            return Err(SaveStateError::CouldNotSaveToFile);
        }

        state_file.flush().map_err(|_| SaveStateError::CouldNotSaveToFile)?;
        info!("State saved successfully");
        Ok(())
    }*/

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
                if let Err(e) = std::fs::remove_file("state.json") {
                    error!("Could not delete 'state.json' saving state may not be possible: {e}");
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


/*
#[tauri::command(rename_all = "snake_case")]
pub(crate) fn add_mod(path: &str) -> Result<Vec<Mod>, AddModError> {
    info!("Adding mod '{path}'");
    State::get().add_mod(path).map(|mods| {
        info!("Added mod '{path}'"); mods
    }).map_err(|e| {
        error!("Failed to add mod: '{path}': {e:?}"); e
    })
}


#[tauri::command(rename_all = "snake_case")]
pub(crate) fn remove_mod(index: usize) -> Result<Vec<Mod>, RemoveModError> {
    let filename = STATE.lock().remove_mod(index)?;

    let mut package_location = PathBuf::from("Mods");
    package_location.push(filename);

    std::fs::remove_file(package_location)
        .map_err(|_| RemoveModError::ErrorRemovingModFile)?;

    Ok(State::get().get_mods())
}


fn move_mod_file(path: &str) -> Result<String, AddModError> {
    let file_name = Path::new(path).file_name()
        .ok_or(InvalidFilePath(String::from(path)))?;

    let mut destination = PathBuf::from("Mods");
    destination.push(file_name);

    std::fs::copy(path, destination)
        .map_err(|e| AddModError::CouldNotReadFile {
            description: e.to_string()
        })?;
    Ok(file_name.to_str().expect("Mod filename OS string is not utf-8 somehow?").to_string())
}*/