use std::path::PathBuf;
use spin::Mutex;
use crate::models::{Mod, ModDetails};
use crate::state::State;


#[tauri::command]
pub fn create_profile(state: tauri::State<Mutex<State>>, name: String) {
    let mut state = state.inner().lock();
    state.create_profile(name);
}

#[tauri::command]
pub fn get_mods(state: tauri::State<Mutex<State>>) -> Vec<Mod> {
    let state = state.inner().lock();
    state.get_mods().iter().map(|mod_state| {
        match &mod_state.meta {
            Some(meta) => Mod {
                    name: meta.name().value().to_owned(),
                    description: meta.description().to_owned(),
                    enabled: mod_state.enabled,
            },
            None => Mod {
                name: mod_state.path.file_name().map(|file_name| {
                    file_name.to_string_lossy().to_string()
                }).unwrap_or("...".into()),
                description: String::new(),
                enabled: mod_state.enabled,
            },
        }
    }).collect()
}

#[tauri::command]
pub fn get_mod_details(state: tauri::State<Mutex<State>>, path: PathBuf) -> Result<ModDetails, ()> {
    let mut state = state.inner().lock();
    state.get_mod_details(path)
}

#[tauri::command]
pub fn add_current_mod(state: tauri::State<Mutex<State>>) {
    let mut state = state.inner().lock();
    state.add_current_mod();
}