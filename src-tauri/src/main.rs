// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use log::{Level, logger, Record};
use spin::Mutex;
use crate::file_browser::FileBrowser;
use crate::state::State;

mod state;
mod file_browser;
mod helper;
mod models;
mod translation;

fn log(level: Level, message: String, target: String,
       module_path: Option<String>, file: Option<String>, line: Option<u32>) {

    let module_path = module_path.as_ref().map(|s| s.as_str());
    let file = file.as_ref().map(|s| s.as_str());

    let mut binding = Record::builder();
    logger().log(&binding
        .args(format_args!("{}", message))
        .level(level)
        .target(&target)
        .module_path(module_path)
        .file(file)
        .line(line)
        .build());
}

#[tauri::command]
fn info(message: String, target: String,
        module_path: Option<String>, file: Option<String>, line: Option<u32>) {
    log(Level::Info, message, target, module_path, file, line);
}

#[tauri::command]
fn warn(message: String, target: String,
        module_path: Option<String>, file: Option<String>, line: Option<u32>) {
    log(Level::Warn, message, target, module_path, file, line);
}

#[tauri::command]
fn error(message: String, target: String, module_path: Option<String>, file: Option<String>, line: Option<u32>) {
    log(Level::Error, message, target, module_path, file, line);
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
            state::commands::switch_profile,
            state::commands::get_profiles,
            state::commands::get_current_profile,
            state::commands::delete_profile,
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
