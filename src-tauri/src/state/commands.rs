use std::path::PathBuf;
use spin::Mutex;
use crate::models::{ModDetails, ModState};
use crate::state::State;


#[tauri::command]
pub fn create_profile(state: tauri::State<Mutex<State>>, name: String) {
    let mut state = state.inner().lock();
    state.create_profile(name);
}

#[tauri::command]
pub fn get_mods(state: tauri::State<Mutex<State>>) -> Vec<ModState> {
    let state = state.inner().lock();
    state.get_mods().to_vec()
}

#[tauri::command]
pub fn get_mod_details(state: tauri::State<Mutex<State>>, path: PathBuf) -> Result<ModDetails, ()> {
    let mut state = state.inner().lock();
    state.get_mod_details(path)
}