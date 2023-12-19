// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use log::{error, info, warn};
use spin::Mutex;
use crate::file_browser::FileBrowser;
use crate::state::State;

mod state;
mod file_browser;
mod helper;
mod models;
mod translation;

#[tauri::command]
fn info(message: String) {
    info!("UI: {message}");
}

#[tauri::command]
fn warn(message: String) {
    warn!("UI: {message}");
}

#[tauri::command]
fn error(message: String) {
    error!("UI: {message}");
}


fn main() {
    simple_logger::init().unwrap();

    let mut state = State::new();
    state.load();

    tauri::Builder::default()
        .manage(Mutex::new(state))
        .manage(Mutex::new(FileBrowser::new()))
        .invoke_handler(tauri::generate_handler![
            // Logging
            info,
            warn,
            error,

            // State
            state::commands::create_profile,
            state::commands::get_mods,
            state::commands::get_mod_details,
            state::commands::add_current_mod,
            state::commands::apply,
            state::commands::delete,
            state::commands::toggle_mod_enabled,

            // File Browser
            file_browser::commands::redirect_browser,
            file_browser::commands::read_current_dir,
            file_browser::commands::get_common_paths,
            file_browser::commands::go_back,
            file_browser::commands::go_forward,
            file_browser::commands::can_go_back_forward,

            // Translation
            translation::load_translation,

        ])
        //.invoke_handler(register_translation_commands())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
