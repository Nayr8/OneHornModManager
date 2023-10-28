// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::file_browser::FileBrowser;
use crate::logger::Logger;
use crate::state::State;

mod mod_package;
mod state;
mod mod_settings_builder;
mod logger;
mod file_browser;
mod extensions;


fn main() {
    Logger::init();
    FileBrowser::init();
    State::load();
    let mut dir = home::home_dir().expect("Could not find home directory");
    dir.push(".steam/steam/steamapps/compatdata/1086940/pfx/drive_c/users/steamuser/AppData/Local/Larian Studios/Baldur's Gate 3/");
    State::get().bg3_appdata = dir.to_string_lossy().to_string();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            logger::log_trace,
            logger::log_debug,
            logger::log_info,
            logger::log_warn,
            logger::log_error,
            logger::get_log_messages,
            file_browser::read_current_dir,
            file_browser::redirect_browser,
            state::get_mod_details,
            state::get_mods,
            state::add_current_mod,
            state::remove_mod,
            state::save,
            state::apply,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
