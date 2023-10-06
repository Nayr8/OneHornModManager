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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            logger::log_info,
            logger::log_warn,
            logger::log_error,
            logger::get_log_messages,
            file_browser::read_current_dir,
            file_browser::redirect_browser,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
