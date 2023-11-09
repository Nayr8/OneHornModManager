use std::path::PathBuf;
use models::{MMResult, Mod, ModDetailsError};
use crate::state::State;

#[tauri::command(rename_all = "snake_case")]
pub fn get_mods() -> Vec<Mod> {
    State::get_mods()
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_current_mod() {
    State::add_current_mod();
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_mod(index: usize) {
    State::remove_mod(index);
}

#[tauri::command(rename_all = "snake_case")]
pub fn apply() {
    State::apply();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_mod_details(file_path: PathBuf) -> MMResult<Mod, ModDetailsError> {
    State::get_mod_details(file_path).into()
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_mod_enabled_state(index: usize, enabled: bool) {
    State::set_mod_enabled_state(index, enabled);
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_profile(name: String) {
    State::create_profile(name);
}

#[tauri::command(rename_all = "snake_case")]
pub fn switch_profile(index: usize) {
    State::switch_profile(index);
}