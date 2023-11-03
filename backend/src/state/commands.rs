use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use models::{MMResult, Mod, ModDetailsError};
use crate::{debug, error, info, trace};
use crate::extensions::HasExtension;
use crate::state::{get_mod_meta, ModState, State};

#[tauri::command(rename_all = "snake_case")]
pub fn get_mods() -> Vec<Mod> {
    let state = State::get();

    let mut mods = Vec::with_capacity(state.mods.len());
    for mod_state in &state.mods {
        mods.push(
            State::get_mod_details(&mod_state.meta, &PathBuf::from(&mod_state.path))
        );
    }
    mods
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_current_mod() {
    let mut state = State::get();

    let (path, meta) = match state.selected_new_mod_meta.take() {
        Some(meta) => meta,
        None => {
            error!("No mod meta cached");
            return; // TODO return and handle error
        }
    };

    state.mods.push(ModState {
        meta,
        path: path.to_string_lossy().to_string(),
        enabled: true,
    });
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_mod(index: usize) {
    let mut state = State::get();
    state.mods.remove(index);
}

#[tauri::command(rename_all = "snake_case")]
pub fn save() {
    let state = State::get();

    let state_string = match serde_json::to_string::<State>(&state) {
        Ok(state_string) => state_string,
        Err(error) => {
            error!("Could not serialize state: {error:?}");
            return; // TODO return and handle error
        }
    };

    let data_dir = State::get_data_dir();
    let mut file = match OpenOptions::new().write(true).create(true).truncate(true).open(data_dir.join("state.json")) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not save file: {error:?}");
            return; // TODO return and handle error
        }
    };

    if let Err(error) = file.write_all(state_string.as_bytes()) {
        error!("Could not save file: {error:?}");
        return; // TODO return and handle error
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn apply() {
    let mut state = State::get();

    info!("Creating symlinks to mod pak files");
    let mut mods_folder_path = PathBuf::from(&state.bg3_appdata);
    mods_folder_path.push("Mods/");

    for entry in std::fs::read_dir(&mods_folder_path).unwrap() {
        if entry.is_err() { continue }
        let entry = entry.unwrap();

        if let Ok(file_type) = entry.file_type() {
            if file_type.is_symlink() {
                symlink::remove_symlink_auto(entry.path()).unwrap();
            }
        }
    }

    for mod_state in &state.mods {
        if !mod_state.enabled { continue }

        let mut path = mods_folder_path.clone();
        let src_path = PathBuf::from(&mod_state.path);
        path.push(src_path.file_name().expect("Mod file not a file"));

        if let Err(error) = symlink::symlink_file(&mod_state.path, path) {
            error!("Could not apply mod '{}': {error}", mod_state.path);
            return;
        }
    }

    info!("Writing mod settings");
    let mut mod_settings_path = PathBuf::from(&state.bg3_appdata);
    mod_settings_path.push("PlayerProfiles/Public/modsettings.lsx");

    let mod_settings = state.build_mod_settings();

    let mut mod_settings_file = match OpenOptions::new()
        .write(true).create(true).truncate(true)
        .open(&mod_settings_path) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not apply mod_settings: {error:?}");
            return; // TODO return and handle error
        }
    };
    if let Err(error) = mod_settings_file.write_all(mod_settings.as_bytes()) {
        error!("Could not write mod_settings: {error:?}");
        return; // TODO return and handle error
    }
    info!("Mod settings applied")
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_mod_details(file_path: PathBuf) -> MMResult<Mod, ModDetailsError> {
    info!("Fetching mod details");

    if !file_path.has_extension("pak") {
        error!("File {file_path:?} is not a pak file");
        return MMResult::Err(ModDetailsError::FilePathDoesNotLeadToValidFile)
    }

    trace!("Checking cache for the meta data for this package");
    let mut state = State::get();
    if let Some((path, meta)) = state.selected_new_mod_meta.as_ref() {
        if *path == file_path {
            debug!("Retrieved meta from cache");
            let details = State::get_mod_details(&meta, &file_path);

            info!("Returning mod details {{name: {}, description: {}, version: {}}}", details.name, details.description, details.version);
            return MMResult::Ok(details)
        }
    }

    let meta = match get_mod_meta(&file_path) {
        Ok(mut meta) => {
            // FIXME: Find a way to use multiple metas
            meta.drain(..).next()
        },
        Err(error) => return MMResult::Err(error)
    };

    let details = State::get_mod_details(&meta, &file_path);

    state.selected_new_mod_meta = Some((file_path, meta));

    info!("Returning mod details {{name: {}, description: {}, version: {}}}", details.name, details.description, details.version);
    MMResult::Ok(details)
}