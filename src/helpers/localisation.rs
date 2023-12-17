use std::collections::HashMap;
use serde::Serialize;
use tauri_sys::tauri;
use yew::platform::spawn_local;
use yew::UseStateHandle;

#[derive(Default, PartialEq)]
pub struct LocalisationHelper {
    translations: HashMap<String, String>,
}

impl LocalisationHelper {
    pub fn change(lang: String, localisation_helper: UseStateHandle<LocalisationHelper>) {
        #[derive(Serialize)]
        struct Args {
            translation: String,
        }
        spawn_local(async move {
            let Ok(translations) = tauri::invoke("load_translation", &Args { translation: lang }).await else {
                return;
            };

            localisation_helper.set(LocalisationHelper {
                translations,
            })
        })
    }

    pub fn trans(&self, value: &str) -> String {
        self.translations.get(value).map(|t| t.clone()).unwrap_or(value.to_owned())
    }
}