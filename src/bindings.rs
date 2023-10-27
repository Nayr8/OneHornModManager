use std::path::PathBuf;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use yew::UseStateHandle;
use models::{FileBrowserRedirectError, FileEntry, MMResult, Mod, ModDetailsError};
use crate::{error, invoke};


pub struct FileBrowser;

#[derive(Serialize, Deserialize)]
struct RedirectBrowserArgs {
    path: Rc<PathBuf>,
}

impl FileBrowser {
    pub fn redirect_browser(path: Rc<PathBuf>) {
        let args = RedirectBrowserArgs {
            path
        };

        spawn_local(async move {
            let result = invoke("redirect_browser", serde_wasm_bindgen::to_value(&args).unwrap()).await;
            if let Err(e) = serde_wasm_bindgen::from_value::<Result<(), FileBrowserRedirectError>>(result).unwrap() {
                let path = args.path;
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

#[derive(Serialize)]
struct GetModDetailsArgs {
    file_path: Rc<PathBuf>,
}

impl ModManager {
    pub fn remove_mod(index: usize, mods: UseStateHandle<Option<Rc<Vec<Mod>>>>) {
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

    pub fn get_mods(mods: UseStateHandle<Option<Rc<Vec<Mod>>>>) {
        spawn_local(async move {
            let fetched_mods = invoke("get_mods", JsValue::null()).await;

            let fetched_mods = serde_wasm_bindgen::from_value::<Rc<Vec<Mod>>>(fetched_mods).unwrap();

            mods.set(Some(fetched_mods));
        });
    }

    pub fn get_mod_details(current_file: Rc<PathBuf>, mod_details: UseStateHandle<Option<Rc<Mod>>>, mod_details_error: UseStateHandle<Option<ModDetailsError>>) {
        let args = GetModDetailsArgs {
            file_path: current_file
        };

        spawn_local(async move {
            let details_result = invoke("get_mod_details", serde_wasm_bindgen::to_value(&args).unwrap()).await;

            let details =
                serde_wasm_bindgen::from_value::<MMResult<Rc<Mod>, ModDetailsError>>(details_result).unwrap();

            match details {
                MMResult::Ok(details) => mod_details.set(Some(details)),
                MMResult::Err(error) => mod_details_error.set(Some(error)),
            }
        });
    }
}
