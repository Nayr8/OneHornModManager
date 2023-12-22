use std::path::PathBuf;
use serde::Serialize;
use tauri_sys::tauri;
use yew::platform::spawn_local;
use yew::prelude::*;
use crate::bindings::Null;
use crate::models::{CommonPath, FileEntry};

pub struct FileBrowserBindings;

impl FileBrowserBindings {
    pub fn get_common_paths(common_paths: UseStateHandle<Option<Vec<(CommonPath, PathBuf)>>>) {
        spawn_local(async move {
            let returned_common_paths: Vec<(CommonPath, PathBuf)> = tauri::invoke("get_common_paths", &Null).await.unwrap();
            common_paths.set(Some(returned_common_paths));
        })
    }

    pub fn read_current_dir(current_directory: UseStateHandle<Option<(String, Vec<FileEntry>)>>, can_go_back_forward: UseStateHandle<Option<(bool, bool)>>) {
        spawn_local(async move {
            let current_dir = tauri::invoke("read_current_dir", &Null).await.unwrap();
            current_directory.set(Some(current_dir));
            FileBrowserBindings::can_go_back_forward(can_go_back_forward);
        })
    }

    pub fn redirect_browser(path: PathBuf, current_directory: UseStateHandle<Option<(String, Vec<FileEntry>)>>,
                            can_go_back_forward: UseStateHandle<Option<(bool, bool)>>) {
        #[derive(Serialize)]
        struct Args {
            path: PathBuf
        }
        spawn_local(async move {
            let _: () = tauri::invoke("redirect_browser", &Args { path }).await.unwrap();
            Self::read_current_dir(current_directory, can_go_back_forward)
        })
    }

    fn can_go_back_forward(can_go_back_forward: UseStateHandle<Option<(bool, bool)>>) {
        spawn_local(async move {
            let resp = tauri::invoke("can_go_back_forward", &Null).await.unwrap();
            can_go_back_forward.set(Some(resp))
        })
    }

    pub fn go_forward(current_directory: UseStateHandle<Option<(String, Vec<FileEntry>)>>,
                      can_go_back_forward: UseStateHandle<Option<(bool, bool)>>) {
        spawn_local(async move {
            let _: () = tauri::invoke("go_forward", &Null).await.unwrap();
            Self::read_current_dir(current_directory, can_go_back_forward)
        })
    }

    pub fn go_back(current_directory: UseStateHandle<Option<(String, Vec<FileEntry>)>>,
                   can_go_back_forward: UseStateHandle<Option<(bool, bool)>>) {
        spawn_local(async move {
            let _: () = tauri::invoke("go_back", &Null).await.unwrap();
            Self::read_current_dir(current_directory, can_go_back_forward)
        })
    }
}