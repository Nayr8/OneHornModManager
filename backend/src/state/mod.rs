use std::fs::{create_dir_all, File, OpenOptions, remove_dir_all, copy};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use spin::{Mutex, MutexGuard};
use models::{Mod, ModDetailsError};
use package_helper::{Meta, PackageReader};
use crate::{debug, error, info, trace, warn};
use crate::mod_settings_builder::ModSettingsBuilder;
use crate::state::helpers::PathHelper;
use crate::state::mod_models::SelectedNewModInfo;
use crate::state::profiles::Profiles;

pub mod commands;
pub mod mod_models;
mod helpers;
mod profiles;

static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::new()));


#[derive(Serialize, Deserialize)]
pub(crate) struct State {
    #[serde(skip)]
    pub(crate) bg3_appdata: String,

    selected_new_mod_info: Option<SelectedNewModInfo>,

    profiles: Profiles,
    gustav_dev_mod_meta: Option<Meta>,
}

// Loading and saving
impl State {
    pub fn init() {
        let mut state = State::get();
        state.load();
        state.bg3_appdata = PathHelper::find_bg3_app_data().to_string_lossy().to_string();
        state.profiles.init();
    }

    pub fn apply() {
        let mut state = State::get();

        info!("Creating symlinks to mod pak files");
        let mut mods_folder_path = PathBuf::from(&state.bg3_appdata);
        mods_folder_path.push("Mods/");

        for entry in std::fs::read_dir(&mods_folder_path).unwrap() {
            if entry.is_err() { continue }
            let entry = entry.unwrap();

            if let Ok(file_type) = entry.file_type() {
                if file_type.is_symlink() {
                    symlink::remove_symlink_auto(entry.path()).unwrap();
                }
            }
        }

        for mod_state in state.profiles.get_mods() {
            if !mod_state.enabled { continue }

            let mut path = mods_folder_path.clone();


            let mut src_path = None;
            let dir = std::fs::read_dir(&mod_state.path).unwrap();
            for entry in dir {
                let Ok(entry) = entry else { continue };

                if entry.file_name().to_string_lossy().ends_with(".pak") {
                    src_path = Some(PathBuf::from(&mod_state.path).join(entry.file_name()));
                    break;
                }
            }
            let src_path = src_path.unwrap();

            path.push(src_path.file_name().expect("Mod file not a file"));

            if let Err(error) = symlink::symlink_file(&src_path, path) {
                error!("Could not apply mod '{:?}': {error}", src_path);
                return;
            }
        }

        info!("Writing mod settings");
        let mut mod_settings_path = PathBuf::from(&state.bg3_appdata);
        mod_settings_path.push("PlayerProfiles/Public/modsettings.lsx");

        let mod_settings = state.build_mod_settings();

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
        info!("Mod settings applied");
    }

    pub fn get_mods() -> Vec<Mod> {
        let state = State::get();

        state.profiles.get_mods().iter().map(|mod_state| {
            State::meta_to_mod_details(mod_state.meta.as_ref(), &PathBuf::from(&mod_state.path), mod_state.enabled)
        }).collect::<Vec<Mod>>()
    }

    pub fn remove_mod(index: usize) {
        let mut state = State::get();
        state.profiles.remove_mod(index);
        state.save();
    }

    pub fn add_current_mod() {
        let mut state = State::get();


        let Some(mod_info) = state.selected_new_mod_info.take() else {
            error!("No mod info cached");
            return; // TODO return and handle error
        };
        state.profiles.add_mod(&mod_info.unpacked_data, mod_info.meta);

        state.save();
    }

    pub fn set_mod_enabled_state(index: usize, enabled: bool) {
        let mut state = State::get();
        state.profiles.set_mod_enabled_state(index, enabled);
        state.save();
    }

    pub fn get_mod_details(file_path: PathBuf) -> Result<Mod, ModDetailsError> {
        info!("Fetching mod details");
        let mut state = State::get();

        trace!("Checking cache for the meta data for this package");
        if let Some(meta) = state.try_get_meta_from_cache(&file_path) {
            debug!("Retrieved meta from cache");
            let details = State::meta_to_mod_details(meta, &file_path, true);

            info!("Returning mod details {{name: {}, description: {}, version: {}}}", details.name, details.description, details.version);
            return Ok(details)
        }

        let extension = file_path.extension().map(std::ffi::OsStr::to_string_lossy);
        let data_path = match extension.as_ref().map(std::convert::AsRef::as_ref) {
            Some("pak") => state.mov_pak(&file_path),
            Some("zip") => state.extract_zip(&file_path),
            _ => {
                error!("File {file_path:?} does not have a supported extension");
                return Err(ModDetailsError::FilePathDoesNotLeadToValidFile)
            }
        };

        let meta = match State::get_mod_metas(&data_path) {
            Ok(mut meta) => {
                // FIXME: Find a way to use multiple metas
                meta.drain(..).next()
            },
            Err(error) => return Err(error)
        };

        let details = State::meta_to_mod_details(meta.as_ref(), &file_path, true);

        state.selected_new_mod_info = Some(SelectedNewModInfo::new(file_path, meta, data_path));
        state.save();

        info!("Returning mod details {{name: {}, description: {}, version: {}}}", details.name, details.description, details.version);
        Ok(details)
    }

    pub fn create_profile(name: String) {
        let mut state = State::get();
        state.profiles.add_profile(name);
    }

    pub fn switch_profile(index: usize) {
        let mut state = State::get();
        state.profiles.switch_profile(index);
    }



    fn new() -> State {
        State {
            selected_new_mod_info: None,
            profiles: Profiles::new(),
            gustav_dev_mod_meta: None,
            bg3_appdata: String::new(),
        }
    }

    fn load(&mut self) {
        info!("Attempting to load state");
        let data_directory = PathHelper::get_data_dir();
        create_dir_all(&data_directory).expect("Could not create data directory");
        let mut state_file = match File::open(data_directory.join("state.json")) {
            Ok(state_file) => state_file,
            Err(e) => if e.kind() == ErrorKind::NotFound {
                info!("No 'state.json' file found");
                return;
            } else {
                error!("Could not open 'state.json' attempting to remove possibly corrupted file: {e}");
                if let Err(e) = std::fs::remove_file(data_directory.join("state.json")) {
                    error!("Could not delete 'state.json' saving state may not be possible: {e}");
                }
                return;
            },
        };

        let mut state_string = String::new();
        if let Err(e) = state_file.read_to_string(&mut state_string) {
            error!("Could not read 'state.json' attempting to remove corrupted file: {e}");
            if let Err(e) = std::fs::remove_file("state.json") {
                error!("Could not delete 'state.json' saving state may not be possible: {e}");
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

        self.profiles = state_data.profiles;
        self.gustav_dev_mod_meta = state_data.gustav_dev_mod_meta;
        self.selected_new_mod_info = state_data.selected_new_mod_info;
        info!("State loaded successfully");
    }

    fn save(&self) {
        info!("Saving...");
        let state_string = match serde_json::to_string::<State>(self) {
            Ok(state_string) => state_string,
            Err(error) => {
                error!("Could not serialize state: {error:?}");
                return; // TODO return and handle error
            }
        };

        let data_dir = PathHelper::get_data_dir();
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

    fn get() -> MutexGuard<'static, State> {
        STATE.lock()
    }

    fn build_mod_settings(&mut self) -> String {
        let gustav_dev_meta = if let Some(gustav_dev_meta) = self.gustav_dev_mod_meta.as_ref() { gustav_dev_meta } else {
            self.gustav_dev_mod_meta = Some(Meta::gustav_dev());
            self.gustav_dev_mod_meta.as_ref().unwrap()
        };

        let xml = ModSettingsBuilder::build(self.profiles.get_mods(), gustav_dev_meta);

        let mut a = Vec::new();
        xml.generate(&mut a).unwrap();

        String::from_utf8(a).unwrap()
    }

    fn try_get_meta_from_cache(&mut self, file_path: &Path) -> Option<Option<&Meta>> {
        // Code is formatted weirdly due to lifetime fuckery with early returns
        // See https://github.com/rust-lang/rust/issues/54663
        trace!("Checking cache for the meta data for this package");
        let meta_valid = if let Some(mod_info) = self.selected_new_mod_info.as_ref() {
            mod_info.src_path() == file_path
        } else { false };

        if meta_valid {
            self.selected_new_mod_info.as_ref().map(SelectedNewModInfo::meta)
        } else {
            if self.selected_new_mod_info.is_some() {
                self.clear_mod_addition_cache();
                self.save();
            }
            None
        }
    }

    fn clear_mod_addition_cache(&mut self) {
        let Some(mod_info) = self.selected_new_mod_info.take() else { return };

        let Some(file_name) = mod_info.src_path().file_stem() else { return };

        let mut mod_store = PathHelper::get_mod_store_dir();
        mod_store.push(file_name);

        let cache_is_duplicate = self.profiles.get_mods().iter().any(|state| {
            state.path == mod_store
        });

        if !cache_is_duplicate && remove_dir_all(&mod_store).is_err() {
            warn!("Could not remove mod data dir for caches data {}", mod_store.to_string_lossy());
        }
    }

    fn meta_to_mod_details(meta: Option<&Meta>, file_path: &Path, enabled: bool) -> Mod {
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

    fn mov_pak(&self, file_path: &Path) -> PathBuf {
        let data_dir_path = self.profiles.calculate_extraction_path(file_path);

        create_dir_all(&data_dir_path).unwrap();

        copy(file_path, data_dir_path.join(file_path.file_name().unwrap())).unwrap();

        data_dir_path
    }

    fn extract_zip(&self, file_path: &Path) -> PathBuf {
        let data_dir_path = self.profiles.calculate_extraction_path(file_path);

        create_dir_all(&data_dir_path).unwrap();

        let src = File::open(file_path).unwrap();
        trace!("Unzipping file");
        zip_extract::extract(src, &data_dir_path, true).unwrap();
        trace!("File Unzipped");
        data_dir_path
    }

    fn get_mod_metas(dir_path: &Path) -> Result<Vec<Meta>, ModDetailsError> {
        let file_path = State::find_pak_path(dir_path).ok_or_else(|| {
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
}