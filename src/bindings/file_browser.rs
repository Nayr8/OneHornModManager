use std::path::PathBuf;
use tauri_sys::tauri;
use yew::platform::spawn_local;
use yew::prelude::*;
use crate::bindings::Null;
use crate::models::CommonPath;

pub struct FileBrowserBindings;

impl FileBrowserBindings {
    pub fn get_common_paths(common_paths: UseStateHandle<Option<Vec<(CommonPath, PathBuf)>>>) {
        spawn_local(async move {
            let returned_common_paths: Vec<(CommonPath, PathBuf)> = tauri::invoke("get_common_paths", &Null).await.unwrap();
            common_paths.set(Some(returned_common_paths))
        })
    }
}