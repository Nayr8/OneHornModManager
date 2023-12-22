use std::fs::{copy, create_dir_all, File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use log::{debug, error, info, trace, warn};
use package_helper::{Meta, PackageReader};
use crate::helper::PathHelper;
use crate::models::{ModDetails, ModState, SelectedNewModInfo};
use crate::state::mod_settings_builder::ModSettingsBuilder;
use crate::state::profile::Profile;

mod profile;
pub mod commands;
mod mod_settings_builder;

#[derive(Serialize, Deserialize)]
pub struct State {
    current_profile: Uuid,
    profiles: HashMap<Uuid, Profile>,

    selected_new_mod_info: Option<SelectedNewModInfo>,

    #[serde(skip, default="PathHelper::find_bg3_app_data")]
    bg3_appdata: PathBuf,
    #[serde(skip, default="Meta::gustav_dev")]
    gustav_dev_mod_meta: Meta,

    #[serde(default)]
    mod_data_to_remove_on_apply: Vec<PathBuf>,
}

impl State {
    pub fn new() -> State {
        State {
            current_profile: Uuid::nil(),
            profiles: HashMap::new(),
            selected_new_mod_info: None,
            bg3_appdata: PathHelper::find_bg3_app_data(),
            gustav_dev_mod_meta: Meta::gustav_dev(),
            mod_data_to_remove_on_apply: Vec::new(),
        }
    }

    pub fn load(&mut self) {
        let data_directory = PathHelper::get_data_dir();
        create_dir_all(&data_directory).expect("Could not create data directory");
        let mut state_file = match File::open(data_directory.join("state.json")) {
            Ok(state_file) => state_file,
            Err(e) => if e.kind() == ErrorKind::NotFound {
                info!("No 'state.json' file found");
                self.create_profile("Default".into());
                return;
            } else {
                error!("Could not open 'state.json' attempting to remove possibly corrupted file: {e}");
                if let Err(e) = std::fs::remove_file(data_directory.join("state.json")) {
                    error!("Could not delete 'state.json' saving state may not be possible: {e}");
                }
                self.create_profile("Default".into());
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
                self.create_profile("Default".into());
                return;
            },
        };

        *self = state_data;
    }

    pub fn create_profile(&mut self, name: String) {
        self.current_profile = Uuid::new_v4();
        self.profiles.insert(self.current_profile, Profile::new(name));
    }

    pub fn switch_profile(&mut self, profile: Uuid) {
        if self.profiles.contains_key(&profile) {
            self.current_profile = profile;
        } else {
            error!("Could not switch profile. '{profile}' is not a profile")
        }
    }

    pub fn delete_profile(&mut self, profile_id: Uuid) {
        if self.current_profile == profile_id {
            error!("Cannot delete active profile");
            return;
        }

        match self.profiles.get(&profile_id) {
            Some(profile) => {
                for mod_state in profile.get_mods() {
                    self.mod_data_to_remove_on_apply.push(mod_state.path.clone());
                }
                self.profiles.remove(&profile_id);
            },
            None => {
                error!("Tried to delete profile {profile_id} but it does not exist");
            }
        }
    }

    pub fn get_profiles(&self) -> Vec<(Uuid, String)> {
        self.profiles.iter().filter(|(id, _)| **id != self.current_profile)
            .map(|(id, profile)| (*id, profile.name().to_owned())).collect()
    }

    pub fn get_current_profile(&self) -> (Uuid, String) {
        (self.current_profile, self.profiles.get(&self.current_profile).unwrap().name().to_owned())
    }

    pub fn get_mods(&self) -> &[ModState] {
        self.profiles[&self.current_profile].get_mods()
    }

    pub fn get_mod_details(&mut self, path: PathBuf) -> Result<ModDetails, ()> {
        if let Some(mod_details) = self.try_get_cached_details(&path) {
            return Ok(mod_details);
        }

        let extension = path.extension().map(std::ffi::OsStr::to_string_lossy);
        let data_path = match extension.as_ref().map(std::convert::AsRef::as_ref) {
            Some("pak") => self.mov_pak(&path).map_err(|_| {
                error!("Error moving .pak file");
                ()
            })?,
            Some("zip") => self.extract_zip(&path).map_err(|_| {
                error!("Error unpacking .zip file");
                ()
            })?,
            _ => {
                error!("Tried to get mod details from an unsupported extension");
                return Err(())
            },
        };

        let meta = match State::get_mod_metas(&data_path) {
            Ok(mut meta) => {
                // FIXME: Find a way to use multiple metas
                meta.drain(..).next()
            },
            Err(()) => {
                error!("Error occurred trying to get meta");
                return Err(())
            }
        };

        self.selected_new_mod_info = Some(SelectedNewModInfo {
            src_path: path.clone(),
            meta,
            unpacked_data: data_path,
        });
        self.save();

        Ok(match self.selected_new_mod_info.as_ref().and_then(|info| info.meta.as_ref()) {
            Some(meta) => ModDetails {
                name: meta.name().value().into(),
                description: meta.description().into(),
            },
            None => ModDetails {
                name: path.file_name().map(|name| name.to_string_lossy().to_string()).unwrap_or("...".into()),
                description: String::new(),
            },
        })
    }

    fn mov_pak(&self, path: &Path) -> Result<PathBuf, ()> {
        let data_dir_path = self.calculate_extraction_path(path);

        create_dir_all(&data_dir_path).unwrap();

        copy(path, data_dir_path.join(path.file_name().ok_or(())?)).map_err(|_| ())?;

        Ok(data_dir_path)
    }

    fn extract_zip(&self, path: &Path) -> Result<PathBuf, ()> {
        let data_dir_path = self.calculate_extraction_path(path);

        create_dir_all(&data_dir_path).map_err(|_| ())?;

        let src = File::open(path).map_err(|_| ())?;
        zip_extract::extract(src, &data_dir_path, true).map_err(|_| ())?;
        Ok(data_dir_path)
    }

    fn try_get_cached_details(&self, path: &Path) -> Option<ModDetails> {
        let Some(selected_new_mod_info) = &self.selected_new_mod_info else {
            return None;
        };
        if selected_new_mod_info.src_path != path { return None; }

        Some(match &selected_new_mod_info.meta {
            Some(meta) => ModDetails {
                name: meta.name().value().into(),
                description: meta.description().into(),
            },
            None => ModDetails {
                name: path.file_name().map(|name| name.to_string_lossy().to_string()).unwrap_or("...".into()),
                description: String::new(),
            },
        })
    }

    fn calculate_extraction_path(&self, src_path: &Path) -> PathBuf {
        let mut path = PathHelper::get_mod_store_dir();
        path.push(self.current_profile.to_string());
        path.push(src_path.file_stem().unwrap());

        path
    }

    fn get_mod_metas(dir_path: &Path) -> Result<Vec<Meta>, ()> {
        let file_path = State::find_pak_path(dir_path).ok_or(())?;

        let package = PackageReader::read_package(&file_path).map_err(|_| ())?;

        package.get_meta().map_err(|_| ())
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

    fn save(&self) {
        info!("Saving...");
        let Ok(state_string) = serde_json::to_string::<State>(self) else { return };// TODO return and handle error


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
    }

    fn add_current_mod(&mut self) {
        let Some(selected_new_mod_info) = self.selected_new_mod_info.take() else {
            warn!("Tried to add mod that selected mod when no mod was selected");
            return;
        };

        let Some(profile) = self.profiles.get_mut(&self.current_profile) else {
            error!("Could not get current profile {}", self.current_profile);
            return;
        };

        profile.add_mod(ModState {
            meta: selected_new_mod_info.meta,
            path: selected_new_mod_info.unpacked_data,
            enabled: true,
        });
        self.save();
    }

    // FIXME: IF the paths don't exist anymore there will be a partial crash
    pub fn apply(&mut self) {
        info!("Creating symlinks to mod pak files");
        let mut mods_folder_path = PathBuf::from(&self.bg3_appdata);
        mods_folder_path.push("Mods/");

        trace!("Removing symlinks from Mods folder '{}'", mods_folder_path.to_string_lossy());
        for entry in std::fs::read_dir(&mods_folder_path).unwrap() {
            if entry.is_err() { continue }
            let entry = entry.unwrap();

            if let Ok(file_type) = entry.file_type() {
                if file_type.is_symlink() {
                    symlink::remove_symlink_auto(entry.path()).unwrap();
                }
            }
        }

        for mod_state in self.get_mods() {
            if !mod_state.enabled { continue }

            let mut path = mods_folder_path.clone();

            let mut src_path = None;
            debug!("Reading packages for mod as {}", mod_state.path.to_string_lossy());
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
        let mut mod_settings_path = PathBuf::from(&self.bg3_appdata);
        mod_settings_path.push("PlayerProfiles/Public/modsettings.lsx");

        let mod_settings = self.build_mod_settings();

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

        if !self.mod_data_to_remove_on_apply.is_empty() {
            for path in self.mod_data_to_remove_on_apply.drain(..) {
                if let Err(error) = std::fs::remove_dir_all(&path) {
                    error!("Could not remove mod data from {}: {error:?}", path.to_string_lossy());
                }
            }
            self.save();
        }
        info!("Mod settings applied");
    }

    fn build_mod_settings(&mut self) -> String {
        let xml = ModSettingsBuilder::build(self.get_mods(), &self.gustav_dev_mod_meta);

        let mut a = Vec::new();
        xml.generate(&mut a).unwrap();

        String::from_utf8(a).unwrap()
    }

    pub fn delete_mod(&mut self, mod_index: usize) {
        let Some(profile) = self.profiles.get_mut(&self.current_profile) else {
            error!("Could not get current profile");
            return;
        };

        let Ok(mod_state) = profile.remove_mod(mod_index) else {
            return;
        };

        self.mod_data_to_remove_on_apply.push(mod_state.path);
        self.save();
    }

    pub fn toggle_mod_enabled(&mut self, mod_index: usize) {
        let Some(profile) = self.profiles.get_mut(&self.current_profile) else {
            error!("Could not get current profile");
            return;
        };

        profile.toggle_mod_enabled(mod_index);
    }
}