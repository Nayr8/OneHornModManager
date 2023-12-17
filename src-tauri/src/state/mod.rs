use std::fs::{copy, create_dir_all, File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use log::{error, info};
use package_helper::{Meta, PackageReader};
use crate::helper::PathHelper;
use crate::models::{ModDetails, ModState, SelectedNewModInfo};
use crate::state::profile::Profile;

mod profile;
pub mod commands;

#[derive(Serialize, Deserialize)]
pub struct State {
    current_profile: Uuid,
    profiles: HashMap<Uuid, Profile>,

    selected_new_mod_info: Option<SelectedNewModInfo>,
}

impl State {
    pub fn new() -> State {
        State {
            current_profile: Uuid::nil(),
            profiles: HashMap::new(),
            selected_new_mod_info: None,
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

    pub fn get_mods(&self) -> &[ModState] {
        self.profiles[&self.current_profile].get_mods()
    }

    pub fn get_mod_details(&mut self, path: PathBuf) -> Result<ModDetails, ()> {
        if let Some(mod_details) = self.try_get_cached_details(&path) {
            return Ok(mod_details);
        }

        let extension = path.extension().map(std::ffi::OsStr::to_string_lossy);
        let data_path = match extension.as_ref().map(std::convert::AsRef::as_ref) {
            Some("pak") => self.mov_pak(&path),
            Some("zip") => self.extract_zip(&path),
            _ => return Err(()),
        };

        let meta = match State::get_mod_metas(&data_path) {
            Ok(mut meta) => {
                // FIXME: Find a way to use multiple metas
                meta.drain(..).next()
            },
            Err(()) => return Err(())
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

    fn mov_pak(&self, path: &Path) -> PathBuf {
        let data_dir_path = self.calculate_extraction_path(path);

        create_dir_all(&data_dir_path).unwrap();

        copy(path, data_dir_path.join(path.file_name().unwrap())).unwrap();

        data_dir_path
    }

    fn extract_zip(&self, path: &Path) -> PathBuf {
        let data_dir_path = self.calculate_extraction_path(path);

        create_dir_all(&data_dir_path).unwrap();

        let src = File::open(path).unwrap();
        zip_extract::extract(src, &data_dir_path, true).unwrap();
        data_dir_path
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
        let Ok(state_string) = serde_json::to_string::<State>(self) else { return };// TODO return and handle error


        let data_dir = PathHelper::get_data_dir();
        let mut file = match OpenOptions::new().write(true).create(true).truncate(true).open(data_dir.join("state.json")) {
            Ok(file) => file,
            Err(error) => {
                error!("Could not save file: {error:?}");
                return; // TODO return and handle error
            }
        };

        if let Err(_) = file.write_all(state_string.as_bytes()) {
            return; // TODO return and handle error
        }
    }

    fn add_current_mod(&mut self) {
        let Some(selected_new_mod_info) = self.selected_new_mod_info.take() else {
            return;
        };

        let Some(profile) = self.profiles.get_mut(&self.current_profile) else {
            return;
        };

        profile.add_mod(ModState {
            meta: selected_new_mod_info.meta,
            path: selected_new_mod_info.unpacked_data,
            enabled: true,
        });
        self.save();
    }
}