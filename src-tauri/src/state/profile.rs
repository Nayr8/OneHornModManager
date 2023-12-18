use log::error;
use serde::{Deserialize, Serialize};
use crate::models::ModState;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    name: String,
    mods: Vec<ModState>,
}

impl Profile {
    pub fn new(name: String) -> Profile {
        Profile {
            name,
            mods: Vec::new(),
        }
    }

    pub fn get_mods(&self) -> &[ModState] {
        self.mods.as_slice()
    }

    pub fn add_mod(&mut self, mod_state: ModState) {
        self.mods.push(mod_state);
    }

    pub fn remove_mod(&mut self, mod_index: usize) -> Result<ModState, ()> {
        if mod_index >= self.mods.len() {
            error!("Tried to remove mod that does not exist");
            return Err(());
        }

        Ok(self.mods.remove(mod_index))
    }

    pub fn toggle_mod_enabled(&mut self, mod_index: usize) {
        let Some(mod_data) = self.mods.get_mut(mod_index) else {
            error!("Tried to remove mod that does not exist");
            return;
        };
        mod_data.enabled = !mod_data.enabled;
    }
}