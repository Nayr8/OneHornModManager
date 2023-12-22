use std::path::PathBuf;
use log::info;
use spin::Mutex;
use crate::file_browser::FileBrowser;
use crate::models::{CommonPath, FileEntry};

#[tauri::command]
pub fn redirect_browser(file_browser: tauri::State<Mutex<FileBrowser>>, path: PathBuf) {
    info!("Navigating browser to {}", path.to_string_lossy());
    let mut file_browser = file_browser.inner().lock();
    file_browser.redirect(path).expect("Failed to redirect");
}

#[tauri::command]
pub fn read_current_dir(file_browser: tauri::State<Mutex<FileBrowser>>) -> (PathBuf, Vec<FileEntry>) {
    info!("Reading current directory");
    let file_browser = file_browser.inner().lock();
    file_browser.read_current_dir()
}

#[tauri::command]
pub fn get_common_paths(file_browser: tauri::State<Mutex<FileBrowser>>) -> Vec<(CommonPath, PathBuf)> {
    info!("Getting common paths");
    let file_browser = file_browser.inner().lock();
    let mut paths = Vec::new();

    if let Some(home_directory) = file_browser.home_directory.as_ref() {
        paths.push((CommonPath::Home, home_directory.clone()));
    }
    if let Some(documents_directory) = file_browser.documents_directory.as_ref() {
        paths.push((CommonPath::Documents, documents_directory.clone()));
    }
    if let Some(downloads_directory) = file_browser.downloads_directory.as_ref() {
        paths.push((CommonPath::Downloads, downloads_directory.clone()));
    }
    if let Some(desktop_directory) = file_browser.desktop_directory.as_ref() {
        paths.push((CommonPath::Desktop, desktop_directory.clone()));
    }
    paths
}

#[tauri::command]
pub fn go_back(file_browser: tauri::State<Mutex<FileBrowser>>) {
    info!("Navigating browser back");
    let mut file_browser = file_browser.inner().lock();
    file_browser.go_back()
}

#[tauri::command]
pub fn go_forward(file_browser: tauri::State<Mutex<FileBrowser>>) {
    info!("Navigating browser forward");
    let mut file_browser = file_browser.inner().lock();
    file_browser.go_forward()
}

#[tauri::command]
pub fn can_go_back_forward(file_browser: tauri::State<Mutex<FileBrowser>>) -> (bool, bool) {
    info!("Getting whether the browser can go forward and backwards");
    let file_browser = file_browser.inner().lock();
    (!file_browser.history.is_empty(), !file_browser.future.is_empty())
}