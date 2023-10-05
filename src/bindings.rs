use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use yew::UseStateHandle;
use models::{FileBrowserRedirectError, FileEntry};
use crate::{error, invoke};


pub struct FileBrowser;

#[derive(Serialize, Deserialize)]
struct RedirectBrowserArgs {
    path: PathBuf
}

impl FileBrowser {
    pub fn redirect_browser(path: PathBuf) {
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

    pub fn read_current_dir_into(path: UseStateHandle<PathBuf>, entries: UseStateHandle<Vec<FileEntry>>) {
        spawn_local(async move {
            let current_dir = invoke("read_current_dir", JsValue::null()).await;
            let (new_path, new_entries) =
                serde_wasm_bindgen::from_value::<(PathBuf, Vec<FileEntry>)>(current_dir).unwrap();

            path.set(new_path);
            entries.set(new_entries);
        });
    }
}
