use std::fs::{copy, create_dir_all, File, OpenOptions, remove_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use models::{MMResult, Mod, ModDetailsError};
use crate::{debug, error, info, trace, warn};
use crate::state::{get_mod_meta, ModState, SelectedNewModInfo, State};

#[tauri::command(rename_all = "snake_case")]
pub fn get_mods() -> Vec<Mod> {
    let state = State::get();

    let mut mods = Vec::with_capacity(state.mods.len());
    for mod_state in &state.mods {
        mods.push(
            State::get_mod_details(mod_state.meta.as_ref(), &PathBuf::from(&mod_state.path), mod_state.enabled)
        );
    }
    mods
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_current_mod() {
    let mut state = State::get();

    let Some(mod_info) = state.selected_new_mod_info.take() else {
        error!("No mod info cached");
        return; // TODO return and handle error
    };

    state.mods.push(ModState::from(mod_info));
    drop(state);
    State::save();
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_mod(index: usize) {
    let mut state = State::get();
    let Some(mod_state) = state.mods.get(index) else {
        error!("Could not find mod at position {index}");
        return;
    };
    
    let has_duplicates = state.mods.iter().enumerate().any(|(mod_index, state)| {
        state.path == mod_state.path && mod_index != index
    });
    if !has_duplicates && remove_dir_all(&mod_state.path).is_err() {
        error!("Could not remove mod data dir {}", &mod_state.path);
    }
    state.mods.remove(index);
    drop(state);
    State::save();
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


        let mut src_path = None;
        let dir = std::fs::read_dir(&mod_state.path).unwrap();
        for entry in dir {
            let Ok(entry) = entry else { continue };

            if entry.file_name().to_string_lossy().ends_with(".pak") {
                src_path = Some(PathBuf::from(&mod_state.path).join(entry.file_name()));
                break;
            }
        }
        let src_path = src_path.unwrap();

        path.push(src_path.file_name().expect("Mod file not a file"));

        if let Err(error) = symlink::symlink_file(&src_path, path) {
            error!("Could not apply mod '{:?}': {error}", src_path);
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
    info!("Mod settings applied");
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_mod_details(file_path: PathBuf) -> MMResult<Mod, ModDetailsError> {
    info!("Fetching mod details");

    trace!("Checking cache for the meta data for this package");
    let mut state = State::get();
    if let Some(mod_info) = state.selected_new_mod_info.as_ref() {
        if mod_info.src_path() == file_path {
            debug!("Retrieved meta from cache");
            let details = State::get_mod_details(mod_info.meta(), &file_path, true);

            info!("Returning mod details {{name: {}, description: {}, version: {}}}", details.name, details.description, details.version);
            return MMResult::Ok(details)
        }

        let mut get_mod_store = State::get_mod_store_dir();
        get_mod_store.push(if let Some(get_mod_store) = file_path.file_stem() { get_mod_store } else {
            error!("File path {} does not contain a file name", file_path.to_string_lossy());
            return MMResult::Err(ModDetailsError::FilePathDoesNotLeadToValidFile)
        });
        let get_mod_store_str = get_mod_store.to_string_lossy();
        let already_exists = state.mods.iter().any(|state| {
            state.path == get_mod_store_str
        });
        if !already_exists && remove_dir_all(&get_mod_store).is_err() {
            warn!("Could not remove mod data dir for caches data {}", get_mod_store_str);
        }

        state.selected_new_mod_info = None;
    }

    let extension = if let Some(extension) = file_path.extension() { extension.to_string_lossy() } else {
        error!("File {file_path:?} is not a pak or zip file");
        return MMResult::Err(ModDetailsError::FilePathDoesNotLeadToValidFile)
    };

    trace!("Copying data");
    let data_path = match extension.as_ref() {
        "pak" => mov_pak(&file_path),
        "zip" => mov_zip(&file_path),
        _ => {
            error!("File {file_path:?} does not have a supported extension");
            return MMResult::Err(ModDetailsError::FilePathDoesNotLeadToValidFile)
        }
    };

    trace!("Fetching Meta");
    let meta = match get_mod_meta(&data_path) {
        Ok(mut meta) => {
            // FIXME: Find a way to use multiple metas
            meta.drain(..).next()
        },
        Err(error) => return MMResult::Err(error)
    };

    let details = State::get_mod_details(meta.as_ref(), &file_path, true);

    state.selected_new_mod_info = Some(SelectedNewModInfo::new(file_path, meta, data_path));

    info!("Returning mod details {{name: {}, description: {}, version: {}}}", details.name, details.description, details.version);
    MMResult::Ok(details)
}

fn mov_pak(file_path: &Path) -> PathBuf {
    let name = file_path.file_stem().unwrap();
    let mut data_dir_path = State::get_mod_store_dir();
    data_dir_path.push(name);

    create_dir_all(&data_dir_path).unwrap();

    copy(file_path, data_dir_path.join(file_path.file_name().unwrap())).unwrap();

    data_dir_path
}

fn mov_zip(file_path: &Path) -> PathBuf {
    let name = file_path.file_stem().unwrap();
    let mut data_dir_path = State::get_mod_store_dir();
    data_dir_path.push(name);

    create_dir_all(&data_dir_path).unwrap();

    let src = File::open(file_path).unwrap();
    trace!("Unzipping file");
    zip_extract::extract(src, &data_dir_path, true).unwrap();
    trace!("File Unzipped");
    data_dir_path
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_mod_enabled_state(index: usize, enabled: bool) {
    let mut state = State::get();
    let Some(mod_state) = state.mods.get_mut(index) else {
        error!("Could not find mod to disable at position {index}");
        return;
    };

    mod_state.enabled = enabled;
    drop(state);
    State::save();
}