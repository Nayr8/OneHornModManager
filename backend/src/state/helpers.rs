use std::path::PathBuf;

pub struct PathHelper;

impl PathHelper {
    pub fn find_bg3_app_data() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            let mut steam_dir = steamlocate::SteamDir::locate().expect("Could not find the Steam directory"); // TODO Handle this better
            let bg3_steam_app = steam_dir.app(&1086940).unwrap();
            bg3_steam_app.path
                .join("../../compatdata/1086940/pfx/drive_c/users/steamuser/AppData/Local/Larian Studios/Baldur's Gate 3/")
        }
        #[cfg(target_os = "windows")]
        {
            let appdata_local = dirs::config_local_dir().expect("Windows should have a documents folder but it was not found");
            appdata_local.join("Larian Studios/Baldur's Gate 3/")
        }
        #[cfg(target_os = "macos")]
        {
            let documents = dirs::document_dir().expect("MacOs should have a documents folder but it was not found");
            documents.join("Larian Studios/Baldur's Gate 3/")
        }
    }

    pub fn get_data_dir() -> PathBuf {
        let mut data_dir = dirs::data_local_dir().expect("Could not get local data directory");
        data_dir.push("Nayr8'sBG3ModManager");
        data_dir
    }

    pub fn get_mod_store_dir() -> PathBuf {
        let mut dir = PathHelper::get_data_dir();
        dir.push("Mods");
        dir
    }
}