use std::path::PathBuf;
use std::rc::Rc;
use serde::{Serialize, Serializer};
use yew::platform::spawn_local;
use yew::UseStateHandle;
use models::{FileBrowserRedirectError, FileEntry, MMResult, Mod, ModDetailsError, Profiles, Status};
use crate::error;
use tauri_sys::tauri;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub struct Null;

impl Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_none()
    }
}

pub struct FileBrowser;


impl FileBrowser {
    pub fn get_common_paths(common_paths: UseStateHandle<Status<Vec<(String, Rc<PathBuf>)>>>) {
        spawn_local(async move {
            common_paths.set(Status::Loaded(tauri::invoke("get_common_paths", &Null).await.unwrap()));
        });
    }

    pub fn redirect_browser(path: Rc<PathBuf>) {
        #[derive(Serialize)]
        struct Args { path: Rc<PathBuf> }

        spawn_local(async move {
            let result: Result<(), FileBrowserRedirectError> = tauri::invoke("get_common_paths", &Args { path: path.clone() }).await.unwrap();
            if let Err(e) = result {
                error!("Could not redirect to {path:?}: {e:?}");
            }
        });
    }

    pub fn read_current_dir_into(path: UseStateHandle<Rc<PathBuf>>, entries: UseStateHandle<Vec<FileEntry>>) {
        spawn_local(async move {
            let (new_path, new_entries): (Rc<PathBuf>, Vec<FileEntry>) = tauri::invoke("read_current_dir", &Null).await.unwrap();
            path.set(new_path);
            entries.set(new_entries);
        });
    }

    pub fn get_navigation_enabled_state(navigation_enabled_state: UseStateHandle<(bool, bool)>) {
        spawn_local(async move {
            navigation_enabled_state.set(tauri::invoke("can_go_back_forward", &Null).await.unwrap());
        });
    }

    pub fn go_back() {
        spawn_local(async move {
            let _: () = tauri::invoke("go_back", &Null).await.unwrap();
        });
    }

    pub fn go_forward() {
        spawn_local(async move {
            let _: () = tauri::invoke("go_forward", &Null).await.unwrap();
        });
    }
}

pub struct ModManager;


impl ModManager {
    pub fn apply(applying: UseStateHandle<bool>) {
        applying.set(true);
        spawn_local(async move {
            let _: () = tauri::invoke("apply", &Null).await.unwrap();
            applying.set(false);
        });
    }

    pub fn remove_mod(index: usize, mods: UseStateHandle<Status<Rc<Vec<Mod>>>>) {
        #[derive(Serialize)]
        struct Args { index: usize }
        spawn_local(async move {
            let _: () = tauri::invoke("remove_mod", &Args { index }).await.unwrap();
            ModManager::get_mods(mods);
        });
    }

    pub fn add_mod() {
        spawn_local(async {
            let _: () = tauri::invoke("add_current_mod", &Null).await.unwrap();
        });
    }

    pub fn get_mods(mods: UseStateHandle<Status<Rc<Vec<Mod>>>>) {
        spawn_local(async move {
            mods.set(Status::Loaded(tauri::invoke("get_mods", &Null).await.unwrap()));
        });
    }

    pub fn get_mod_details(current_file: Rc<PathBuf>, mod_details: UseStateHandle<Status<Rc<Mod>, ModDetailsError>>) {
        #[derive(Serialize)]
        struct Args { file_path: Rc<PathBuf> }
        spawn_local(async move {
            match tauri::invoke("get_mod_details", &Args { file_path: current_file }).await.unwrap() {
                MMResult::Ok(details) => mod_details.set(Status::Loaded(details)),
                MMResult::Err(error) => mod_details.set(Status::Error(error)),
            }
        });
    }

    pub fn set_mod_enabled_state(index: usize, enabled: bool) {
        #[derive(Serialize)]
        struct Args { index: usize, enabled: bool }
        spawn_local(async move {
            let _: () = tauri::invoke("set_mod_enabled_state", &Args { index, enabled }).await.unwrap();
        });
    }

    pub fn get_profiles(profiles: UseStateHandle<Status<Profiles>>) {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
            pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
        }
        spawn_local(async move {
            // Cannot use tauri sys because of int key hashmap weirdness
            let fetched_profiles = invoke("get_profiles", JsValue::null()).await;

            // A bit hacky but serde_wasm_bindgen will not deserialize a int key hashmap as it thinks they are strings
            let fetched_profiles = js_sys::JSON::stringify(&fetched_profiles).unwrap().as_string().unwrap();
            let fetched_profiles = serde_json::from_str::<Profiles>(&fetched_profiles).unwrap();

            profiles.set(Status::Loaded(fetched_profiles));
        });
    }

    pub fn switch_profile(index: usize) {
        #[derive(Serialize)]
        struct Args { index: usize }
        spawn_local(async move {
            let _: () = tauri::invoke("switch_profile", &Args { index }).await.unwrap();
        });
    }
    
    pub fn create_profile(name: String) {
        #[derive(Serialize)]
        struct Args { name: String }
        spawn_local(async move {
            let _: () = tauri::invoke("switch_profile", &Args { name }).await.unwrap();
        });
    }
}
