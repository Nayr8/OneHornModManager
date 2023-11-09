use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use package_helper::Meta;
use crate::error;
use crate::state::helpers::PathHelper;
use crate::state::mod_models::ModState;

#[derive(Serialize, Deserialize)]
pub struct Profiles {
    current_profile: usize,
    next_profile: usize,
    profiles: HashMap<usize, Profile>
}

impl Profiles {
    pub fn new() -> Profiles {
        Profiles {
            current_profile: 0,
            next_profile: 0,
            profiles: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        if !self.profiles.contains_key(&0) {
            self.add_profile("Default".into());
        }
    }

    pub fn add_mod(&mut self, unpacked_data_path: &Path, meta: Option<Meta>) {
        self.profiles.get_mut(&self.current_profile).unwrap().add_mod(unpacked_data_path, meta);
    }

    pub fn remove_mod(&mut self, mod_index: usize) {
        self.profiles.get_mut(&self.current_profile).unwrap().remove_mod(mod_index);
    }

    pub fn get_mods(&self) -> &[ModState] {
        self.profiles.get(&self.current_profile).unwrap().mods.as_slice()
    }

    pub fn profiles(&self) -> HashMap<usize, &str> {
        let mut profiles = HashMap::new();
        for (index, profile) in &self.profiles {
            profiles.insert(*index, profile.name.as_str());
        }
        profiles
    }

    pub fn calculate_extraction_path(&self, src_path: &Path) -> PathBuf {
        let mut path = PathHelper::get_mod_store_dir();
        path.push(&self.profiles.get(&self.current_profile).unwrap().name);
        path.push(src_path.file_stem().unwrap());

        path
    }

    pub fn add_profile(&mut self, name: String) {
        self.profiles.insert(self.next_profile, Profile::new(name));
        self.current_profile = self.next_profile;
        self.next_profile += 1;
    }

    pub fn switch_profile(&mut self, profile: usize) {
        if self.profiles.contains_key(&profile) {
            self.current_profile = profile;
        }
    }

    pub fn set_mod_enabled_state(&mut self, mod_index: usize, enabled: bool) {
        self.profiles.get_mut(&self.current_profile).unwrap().set_mod_enabled_state(mod_index, enabled);
    }
}


#[derive(Serialize, Deserialize)]
struct Profile {
    name: String,
    mods: Vec<ModState>,
}

impl Profile {
    fn new(name: String) -> Profile {
        Profile {
            name,
            mods: Vec::new(),
        }
    }

    fn add_mod(&mut self, unpacked_data_path: &Path, meta: Option<Meta>) {
        self.mods.push(ModState {
            meta,
            path: unpacked_data_path.to_owned(),
            enabled: true,
        })
    }

    fn remove_mod(&mut self, mod_index: usize) {
        let Some(mod_state) = self.mods.get(mod_index) else {
            error!("Could not find mod at position {mod_index}");
            return;
        };

        let has_duplicates = self.mods.iter().enumerate().any(|(index, state)| {
            state.path == mod_state.path && index != mod_index
        });

        if !has_duplicates && fs::remove_dir_all(&mod_state.path).is_err() {
            error!("Could not remove mod data dir {}", &mod_state.path.to_string_lossy());
        }

        self.mods.remove(mod_index);
    }

    pub fn set_mod_enabled_state(&mut self, mod_index: usize, enabled: bool) {
        let Some(mod_state) = self.mods.get_mut(mod_index) else {
            error!("Could not find mod to disable at position {mod_index}");
            return;
        };

        mod_state.enabled = enabled;
    }
}