use std::fs::File;
use std::path::PathBuf;
use log::error;
use models::{MMResult, Mod};
use crate::mod_package::ModPackage;

#[tauri::command(rename_all = "snake_case")]
pub fn get_mod_details(file_path: PathBuf) -> MMResult<Mod, ()> {

    if let Some(extension) = file_path.extension() {
        let extension = extension.to_string_lossy();
        if extension.as_ref() != "pak" {
            return MMResult::Err(());
        }
    } else {
        return MMResult::Err(());
    }

    let file = match File::open(file_path) {
        Ok(file) => {
            file
        },
        Err(error) => {
            error!("Not a file: {error:?}");
            return MMResult::Err(());
        }
    };

    get_mod_details_inner(file).into()
}

fn get_mod_details_inner(file: File) -> Result<Mod, ()> {
    let package = match ModPackage::new(file) {
        Ok(package) => package,
        Err(error) => {
            error!("Cannot unpack file: {:?}", error);
            return Err(())
        }
    };

    let meta = match package.read_package_meta() {
        Ok(meta) => meta,
        Err(error) => {
            error!("Cannot read package meta: {:?}", error);
            return Err(())
        }
    };

    Ok(Mod {
        name: meta.name.value,
        description: meta.description.value,
    })
}