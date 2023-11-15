use std::path::PathBuf;
use tauri::Window;
use models::{MMResult, Mod, ModDetailsError};
use crate::error;
use crate::state::State;

#[tauri::command(rename_all = "snake_case", async)]
pub fn get_mods() -> Vec<Mod> {
    State::get_mods()
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn add_current_mod() {
    State::add_current_mod();
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn remove_mod(index: usize) {
    State::remove_mod(index);
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn apply() {
    State::apply();
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command(rename_all = "snake_case", async)]
pub fn get_mod_details(file_path: PathBuf) -> MMResult<Mod, ModDetailsError> {
    State::get_mod_details(file_path).into()
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn set_mod_enabled_state(index: usize, enabled: bool) {
    State::set_mod_enabled_state(index, enabled);
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn create_profile(name: String) {
    State::create_profile(name);
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn switch_profile(index: usize) {
    State::switch_profile(index);
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn get_profiles() -> models::Profiles {
    State::get_profiles()
}