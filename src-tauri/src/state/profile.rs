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
}