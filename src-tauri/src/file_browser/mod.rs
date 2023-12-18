use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use log::{error, warn};
use crate::models::{EntryType, FileEntry};

pub mod commands;


pub struct FileBrowser {
    current_directory: Option<PathBuf>,
    home_directory: Option<PathBuf>,
    documents_directory: Option<PathBuf>,
    downloads_directory: Option<PathBuf>,
    desktop_directory: Option<PathBuf>,
    history: Vec<PathBuf>,
    future: Vec<PathBuf>,
}

impl FileBrowser {
    pub fn new() -> FileBrowser {
        let home_directory = dirs::home_dir();
        let documents_directory = dirs::document_dir();
        let downloads_directory = dirs::download_dir();
        let desktop_directory = dirs::desktop_dir();
        let current_directory = Some(home_directory.clone().unwrap_or(PathBuf::from("/")));
        FileBrowser {
            current_directory,
            home_directory,
            documents_directory,
            downloads_directory,
            desktop_directory,
            history: vec![],
            future: vec![],
        }
    }

    fn go_back(&mut self) {
        if let Some(path) = self.history.pop() {
            let mut path = Some(path);
            std::mem::swap(&mut self.current_directory, &mut path);
            self.future.push(path.unwrap());
        } else {
            warn!("Could not navigate browser backwards: no history");
        }
    }

    fn go_forward(&mut self) {
        if let Some(path) = self.future.pop() {
            let mut path = Some(path);
            std::mem::swap(&mut self.current_directory, &mut path);
            self.history.push(path.unwrap());
        } else {
            warn!("Could not navigate browser forwards: no future");
        }
    }

    fn redirect(&mut self, path: PathBuf) -> Result<(), ()> {
        match fs::metadata(&path) {
            Ok(meta) => if !meta.is_dir() {
                error!("'{}' is not a directory", path.to_string_lossy());
                return Err(());
            },
            Err(error) => {
                error!("Could not get metadata from path '{}': {error:?}", path.to_string_lossy());
                return Err(())
            },
        }

        let mut path = Some(path);
        if path == self.current_directory {
            return Ok(());
        }
        std::mem::swap(&mut self.current_directory, &mut path);

        if let Some(path) = path {
            self.history.push(path);
        }
        self.future = Vec::new();
        Ok(())
    }

    fn read_current_dir(&self) -> (PathBuf, Vec<FileEntry>) {
        match self.current_directory.clone() {
            Some(path) => {
                let contents = FileBrowser::read_dir(&path);
                (path, contents)
            },
            None => (PathBuf::from("/"), Vec::new())
        }
    }

    fn read_dir(path: impl AsRef<Path>) -> Vec<FileEntry> {
        let mut entries = Vec::new();
        if let Ok(dir) = fs::read_dir(path) {
            for entry in dir.filter(Result::is_ok).map(Result::unwrap) {
                if let Some(entry) = Self::decode_entry(entry.path()) {
                    entries.push(entry);
                }
            }
        }
        let (mut directories, mut files): (Vec<_>, Vec<_>) = entries.iter().partition(|entry| entry.entry_type == EntryType::Directory);
        directories.sort_by(|dir0, dir1| dir0.file_name.cmp(&dir1.file_name));
        files.sort_by(|file0, file1| file0.file_name.cmp(&file1.file_name));

        directories.append(&mut files);

        directories.iter().map(|entry| (**entry).clone()).collect()
    }

    fn decode_entry(path: PathBuf) -> Option<FileEntry> {
        // .steampath seems to exist for a legacy reason and is never a functional file or directory
        if path.ends_with(".steampath") {
            return None;
        }

        let meta = match fs::metadata(&path) {
            Ok(meta) => meta,
            Err(_) => {
                return None;
            }
        };

        let file_name = if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_string_lossy();
            format!("{file_name}")
        } else {
            return None;
        };

        if meta.is_dir() {
            return Some(FileEntry {
                entry_type: EntryType::Directory,
                path,
                file_name,
            });
        }
        if meta.is_file() {
            // Only accept .pak and .zip files for now
            let extension = Path::new(&file_name).extension();
            if extension.map_or(false, |extension| {
                extension.eq_ignore_ascii_case("pak") || extension.eq_ignore_ascii_case("zip")
            }) {
                return Some(FileEntry {
                    entry_type: EntryType::File,
                    path,
                    file_name,
                });
            }
            return None;
        }

        if !meta.is_symlink() {
            return None;
        }

        match Self::resolve_symlink(&path) {
            Ok(path) => {
                let mut entry = Self::decode_entry(path)?;
                entry.file_name = file_name.into();
                Some(entry)
            },
            Err(_) => {
                error!("Failed to resolve symlink '{}'", path.to_string_lossy());
                None
            }
        }
    }

    fn resolve_symlink(path: &Path) -> Result<PathBuf, ()> {
        const SYM_LINK_MAX_DEPTH: usize = 10;

        let mut link_path = path.to_owned();
        let mut visited_paths = HashSet::new();
        for _ in 0..SYM_LINK_MAX_DEPTH {
            visited_paths.insert(link_path.clone());

            link_path = fs::read_link(link_path)
                .map_err(|_| ())?;


            if visited_paths.contains(&link_path) {
                return Err(());
            }

            let meta = fs::metadata(&link_path)
                .map_err(|_| ())?;
            if !meta.is_symlink() {
                return Ok(link_path)
            }
        }

        Err(())
    }
}