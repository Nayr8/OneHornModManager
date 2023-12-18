use std::path::PathBuf;
use log::info;
use spin::Mutex;
use crate::models::{Mod, ModDetails};
use crate::state::State;


#[tauri::command]
pub fn create_profile(state: tauri::State<Mutex<State>>, name: String) {
    info!("Creating new profile: {name}");
    let mut state = state.inner().lock();
    state.create_profile(name);
}

#[tauri::command]
pub fn get_mods(state: tauri::State<Mutex<State>>) -> Vec<Mod> {
    info!("Getting mods");
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
    info!("Getting mod details from mod at path: {}", path.to_string_lossy());
    let mut state = state.inner().lock();
    state.get_mod_details(path)
}

#[tauri::command]
pub fn add_current_mod(state: tauri::State<Mutex<State>>) {
    info!("Adding current mod");
    let mut state = state.inner().lock();
    state.add_current_mod();
}

#[tauri::command]
pub fn apply(state: tauri::State<Mutex<State>>) {
    info!("Applying mods");
    let mut state = state.inner().lock();
    state.apply();
}

#[tauri::command]
pub fn delete(state: tauri::State<Mutex<State>>, index: usize) {
    info!("Deleting mod {index}");
    let mut state = state.inner().lock();
    state.delete_mod(index);
}

#[tauri::command]
pub fn toggle_mod_enabled(state: tauri::State<Mutex<State>>, index: usize) {
    info!("Toggling mod enabled status {index}");
    let mut state = state.inner().lock();
    state.toggle_mod_enabled(index);
}