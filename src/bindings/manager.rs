use std::path::PathBuf;
use serde::Serialize;
use tauri_sys::tauri;
use yew::platform::spawn_local;
use yew::UseStateHandle;
use crate::bindings::Null;
use crate::models::{Mod, ModDetails};
use crate::Status;

pub struct ManagerBindings;

impl ManagerBindings {
    pub fn get_mod_details(mod_details: UseStateHandle<Status<ModDetails, ()>>, path: PathBuf) {
        #[derive(Serialize)]
        struct Args {
            path: PathBuf
        }
        spawn_local(async move {
            match tauri::invoke::<_, ModDetails>("get_mod_details", &Args { path }).await {
                Ok(details) => mod_details.set(Status::Loaded(details)),
                Err(tauri_sys::Error::Command(_)) => mod_details.set(Status::Error(())),
                Err(_) => panic!(),
            }
        })
    }

    pub fn add_current_mod() {
        spawn_local(async {
            let _: () = tauri::invoke("add_current_mod", &Null).await.unwrap();
        })
    }

    pub fn get_mods(mods: UseStateHandle<Status<Vec<Mod>, ()>>) {
        spawn_local(async move {
            let mod_details = tauri::invoke("get_mods", &Null).await.unwrap();
            mods.set(Status::Loaded(mod_details));
        })
    }

    pub fn apply() {
        spawn_local(async move {
            let _: () = tauri::invoke("apply", &Null).await.unwrap();
        })
    }

    pub fn delete(index: usize, mods: UseStateHandle<Status<Vec<Mod>, ()>>) {
        #[derive(Serialize)]
        struct Args {
            index: usize,
        }
        spawn_local(async move {
            let _: () = tauri::invoke("delete", &Args { index }).await.unwrap();
            ManagerBindings::get_mods(mods);
        })
    }

    pub fn toggle_mod_enabled(index: usize, mods: UseStateHandle<Status<Vec<Mod>, ()>>) {
        #[derive(Serialize)]
        struct Args {
            index: usize,
        }
        spawn_local(async move {
            let _: () = tauri::invoke("toggle_mod_enabled", &Args { index }).await.unwrap();
            ManagerBindings::get_mods(mods);
        })
    }
}