use std::fs::File;
use std::path::{Path, PathBuf};
use spin::Mutex;
use models::{MMResult, Mod, ModDetailsError};
use crate::extensions::HasExtension;
use crate::{debug, error, info, trace};
use crate::mod_package::{ModMeta, ModPackage};

static CURRENT_MOD_META: Mutex<Option<(PathBuf, ModMeta)>> = Mutex::new(None);

#[tauri::command(rename_all = "snake_case")]
pub fn get_mod_details(file_path: PathBuf) -> MMResult<Mod, ModDetailsError> {
    info!("Fetching mod details");

    if !file_path.has_extension("pak") {
        error!("File {file_path:?} is not a pak file");
        return MMResult::Err(ModDetailsError::FilePathDoesNotLeadToValidFile)
    }

    trace!("Checking cache for the meta data for this package");
    let mut current_mod_meta = CURRENT_MOD_META.lock();
    if let Some((path, meta)) = current_mod_meta.as_ref() {
        if *path == file_path {
            debug!("Retrieved meta from cache");
            let details = Mod {
                name: meta.name.value.clone(),
                description: meta.description.value.clone(),
            };
            info!("Returning mod details {{name: {}, description: {}}}", details.name, details.description);
            return MMResult::Ok(details)
        }
    }

    let meta = match get_mod_meta(&file_path) {
        Ok(meta) => meta,
        Err(error) => return MMResult::Err(error)
    };

    let details = Mod {
        name: meta.name.value.clone(),
        description: meta.description.value.clone(),
    };

    *current_mod_meta = Some((file_path, meta));

    info!("Returning mod details {{name: {}, description: {}}}", details.name, details.description);
    MMResult::Ok(details)
}

fn get_mod_meta(file_path: &Path) -> Result<ModMeta, ModDetailsError> {
    trace!("Attempting to retrieve meta data from path: {file_path:?}");
    let file = File::open(file_path).map_err(|error| {
        error!("Not a file: {error:?}");
        ModDetailsError::FilePathDoesNotLeadToValidFile
    })?;
    trace!("Successfully opened file: {file_path:?}");

    ModPackage::new(file).map_err(|error| {
        error!("Cannot unpack file: {:?}", error);
        ModDetailsError::CannotUnpackPackageFile
    })?.read_package_meta().map_err(|error| {
        error!("Cannot read package meta: {:?}", error);
        ModDetailsError::CannotReadPackageMeta
    })
}