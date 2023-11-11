use std::path::PathBuf;
use std::rc::Rc;
use serde::Serialize;
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use yew::UseStateHandle;
use models::{FileBrowserRedirectError, FileEntry, MMResult, Mod, ModDetailsError, Profiles, Status};
use crate::{error, invoke};


pub struct FileBrowser;


impl FileBrowser {
    pub fn get_common_paths(common_paths: UseStateHandle<Status<Vec<(String, Rc<PathBuf>)>>>) {
        spawn_local(async move {
            let fetched_common_paths = invoke("get_common_paths", JsValue::null()).await;
            let fetched_common_paths = serde_wasm_bindgen::from_value::<Vec<(String, Rc<PathBuf>)>>(fetched_common_paths).unwrap();

            common_paths.set(Status::Loaded(fetched_common_paths));
        });
    }

    pub fn redirect_browser(path: Rc<PathBuf>) {
        #[derive(Serialize)]
        struct Args {
            path: Rc<PathBuf>,
        }

        spawn_local(async move {
            let result = invoke("redirect_browser", serde_wasm_bindgen::to_value(&Args { path: path.clone() }).unwrap()).await;
            if let Err(e) = serde_wasm_bindgen::from_value::<Result<(), FileBrowserRedirectError>>(result).unwrap() {
                error!("Could not redirect to {path:?}: {e:?}");
            }
        });
    }

    pub fn read_current_dir_into(path: UseStateHandle<Rc<PathBuf>>, entries: UseStateHandle<Vec<FileEntry>>) {
        spawn_local(async move {
            let current_dir = invoke("read_current_dir", JsValue::null()).await;
            let (new_path, new_entries) =
                serde_wasm_bindgen::from_value::<(Rc<PathBuf>, Vec<FileEntry>)>(current_dir).unwrap();

            path.set(new_path);
            entries.set(new_entries);
        });
    }
}

pub struct ModManager;


impl ModManager {
    pub fn apply(applying: UseStateHandle<bool>) {
        applying.set(true);
        spawn_local(async move {
            invoke("apply", JsValue::null()).await;
            applying.set(false);
        });
    }

    pub fn remove_mod(index: usize, mods: UseStateHandle<Status<Rc<Vec<Mod>>>>) {
        #[derive(Serialize)]
        struct Args {
            index: usize,
        }
        spawn_local(async move {
            invoke("remove_mod", serde_wasm_bindgen::to_value(&Args { index }).unwrap()).await;
            ModManager::get_mods(mods);
        });
    }

    pub fn add_mod() {
        spawn_local(async {
            invoke("add_current_mod", JsValue::null()).await;
        });
    }

    pub fn get_mods(mods: UseStateHandle<Status<Rc<Vec<Mod>>>>) {
        spawn_local(async move {
            let fetched_mods = invoke("get_mods", JsValue::null()).await;

            let fetched_mods = serde_wasm_bindgen::from_value::<Rc<Vec<Mod>>>(fetched_mods).unwrap();

            mods.set(Status::Loaded(fetched_mods));
        });
    }

    pub fn get_mod_details(current_file: Rc<PathBuf>, mod_details: UseStateHandle<Status<Rc<Mod>, ModDetailsError>>) {
        #[derive(Serialize)]
        struct Args {
            current_file: Rc<PathBuf>,
        }
        spawn_local(async move {
            let details_result = invoke("get_mod_details", serde_wasm_bindgen::to_value(&Args { current_file }).unwrap()).await;

            let details =
                serde_wasm_bindgen::from_value::<MMResult<Rc<Mod>, ModDetailsError>>(details_result).unwrap();

            match details {
                MMResult::Ok(details) => mod_details.set(Status::Loaded(details)),
                MMResult::Err(error) => mod_details.set(Status::Error(error)),
            }
        });
    }

    pub fn set_mod_enabled_state(index: usize, enabled: bool) {
        #[derive(Serialize)]
        struct Args {
            index: usize,
            enabled: bool
        }
        spawn_local(async move {
            invoke("set_mod_enabled_state", serde_wasm_bindgen::to_value(&Args {
                index,
                enabled,
            }).unwrap()).await;
        });
    }

    pub fn get_profiles(profiles: UseStateHandle<Status<Profiles>>) {
        spawn_local(async move {
            let fetched_profiles = invoke("get_profiles", JsValue::null()).await;

            // A bit hacky but serde_wasm_bindgen will not deserialize a int kiy hashmap as it thinks they are strings
            let fetched_profiles = js_sys::JSON::stringify(&fetched_profiles).unwrap().as_string().unwrap();
            let fetched_profiles = serde_json::from_str::<Profiles>(&fetched_profiles).unwrap();

            profiles.set(Status::Loaded(fetched_profiles));
        });
    }

    pub fn switch_profile(index: usize) {
        #[derive(Serialize)]
        struct Args {
            index: usize
        }
        spawn_local(async move {
            invoke("switch_profile", serde_wasm_bindgen::to_value(&Args { index }).unwrap()).await;
        });
    }
}
