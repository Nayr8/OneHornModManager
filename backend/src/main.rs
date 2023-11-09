// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::file_browser::FileBrowser;
use crate::logger::Logger;
use crate::state::State;

//mod mod_package;
mod state;
mod mod_settings_builder;
mod logger;
mod file_browser;

fn main() {
    Logger::init();
    FileBrowser::init();
    State::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            logger::log_trace,
            logger::log_debug,
            logger::log_info,
            logger::log_warn,
            logger::log_error,
            logger::log_critical,
            logger::get_log_messages,
            file_browser::read_current_dir,
            file_browser::redirect_browser,
            file_browser::get_common_paths,
            state::commands::get_mod_details,
            state::commands::get_mods,
            state::commands::add_current_mod,
            state::commands::remove_mod,
            state::commands::apply,
            state::commands::set_mod_enabled_state,
            state::commands::create_profile,
            state::commands::switch_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
