use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use spin::{Mutex, MutexGuard};
use models::{EntryType, FileBrowserRedirectError, FileEntry, MMResult};
use crate::warn;

static FILE_BROWSER: Mutex<FileBrowser> = Mutex::new(FileBrowser {
    current_directory: None,
    home_directory: None,
    documents_directory: None,
    downloads_directory: None,
    desktop_directory: None,
    history: Vec::new(),
    future: Vec::new(),
});

#[tauri::command(rename_all = "snake_case")]
pub fn redirect_browser(path: String) -> MMResult<(), FileBrowserRedirectError> {
    FileBrowser::redirect(PathBuf::from(path)).into()
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn read_current_dir() -> (PathBuf, Vec<FileEntry>) {
    FileBrowser::read_current_dir()
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn get_common_paths() -> Vec<(String, PathBuf)> {
    let mut paths = Vec::new();
    let file_browser = FileBrowser::get();

    if let Some(home_directory) = file_browser.home_directory.as_ref() {
        paths.push(("Home".into(), home_directory.clone()));
    }
    if let Some(documents_directory) = file_browser.documents_directory.as_ref() {
        paths.push(("Documents".into(), documents_directory.clone()));
    }
    if let Some(downloads_directory) = file_browser.downloads_directory.as_ref() {
        paths.push(("Downloads".into(), downloads_directory.clone()));
    }
    if let Some(desktop_directory) = file_browser.desktop_directory.as_ref() {
        paths.push(("Desktop".into(), desktop_directory.clone()));
    }

    paths
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn go_back() {
    FileBrowser::go_back();
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn go_forward() {
    FileBrowser::go_forward();
}

#[tauri::command(rename_all = "snake_case", async)]
pub fn can_go_back_forward() -> (bool, bool) {
    let file_browser = FileBrowser::get();
    (!file_browser.history.is_empty(), !file_browser.future.is_empty())
}

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
    fn get() -> MutexGuard<'static, FileBrowser> {
        FILE_BROWSER.lock()
    }

    pub fn init() {
        let mut file_browser = FileBrowser::get();

        file_browser.home_directory = dirs::home_dir();
        file_browser.documents_directory = dirs::document_dir();
        file_browser.downloads_directory = dirs::download_dir();
        file_browser.desktop_directory = dirs::desktop_dir();
        file_browser.current_directory = Some(file_browser.home_directory.clone().unwrap_or(PathBuf::from("/")));
    }

    fn go_back() {
        let mut file_browser = FileBrowser::get();
        if let Some(path) = file_browser.history.pop() {
            let mut path = Some(path);
            std::mem::swap(&mut file_browser.current_directory, &mut path);
            file_browser.future.push(path.unwrap());
        }
    }

    fn go_forward() {
        let mut file_browser = FileBrowser::get();
        if let Some(path) = file_browser.future.pop() {
            let mut path = Some(path);
            std::mem::swap(&mut file_browser.current_directory, &mut path);
            file_browser.history.push(path.unwrap());
        }
    }

    fn redirect(path: PathBuf) -> Result<(), FileBrowserRedirectError> {
        let mut file_browser = Self::get();

        match fs::metadata(&path) {
            Ok(meta) => if !meta.is_dir() {
                return Err(FileBrowserRedirectError::PathDoesNotLeadToDir);
            },
            Err(_) => return Err(FileBrowserRedirectError::PathDoesNotLeadToDir),
        }

        let mut path = Some(path);
        if path == file_browser.current_directory {
            return Ok(());
        }
        std::mem::swap(&mut file_browser.current_directory, &mut path);

        if let Some(path) = path {
            file_browser.history.push(path);
        }
        file_browser.future = Vec::new();
        Ok(())
    }

    fn read_current_dir() -> (PathBuf, Vec<FileEntry>) {
        match FileBrowser::get().current_directory.clone() {
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

        directories.iter().map(|entry| (*entry).clone()).collect()
    }

    fn decode_entry(path: PathBuf) -> Option<FileEntry> {
        // .steampath seems to exist for a legacy reason and is never a functional file or directory
        if path.ends_with(".steampath") {
            return None;
        }

        let meta = match fs::metadata(&path) {
            Ok(meta) => meta,
            Err(error) => {
                let path = path.display();
                warn!("Cannot determine entry type of {path}: {error}");
                return None;
            }
        };

        let file_name = if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_string_lossy();
            format!("{file_name}")
        } else {
            let path = path.display();
            warn!("Could not find filename from path {path}");
            return None;
        };

        if meta.is_dir() {
            return Some(FileEntry {
                entry_type: EntryType::Directory,
                path: Arc::new(path), // TODO decide whether to use Rc or have a different model in each side
                file_name: Arc::new(file_name),
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
                    path: Arc::new(path),
                    file_name: Arc::new(file_name),
                });
            }
            return None;
        }

        if !meta.is_symlink() {
            let path = path.display();
            warn!("Cannot determine entry type of {path}");
            return None;
        }

        match Self::resolve_symlink(&path) {
            Ok(path) => {
                let mut entry = Self::decode_entry(path)?;
                entry.file_name = file_name.into();
                Some(entry)
            },
            Err(e) => {
                let path = path.display();
                warn!("Symlink {path} could not be resolved: {e}");
                None
            }
        }
    }

    fn resolve_symlink(path: &Path) -> Result<PathBuf, SymlinkResolveError> {
        const SYM_LINK_MAX_DEPTH: usize = 10;

        let mut link_path = path.to_owned();
        let mut visited_paths = HashSet::new();
        for _ in 0..SYM_LINK_MAX_DEPTH {
            visited_paths.insert(link_path.clone());

            link_path = fs::read_link(link_path)
                .map_err(|_| SymlinkResolveError::CouldNotReadSymlink)?;


            if visited_paths.contains(&link_path) {
                return Err(SymlinkResolveError::SymLinkLoopDetected);
            }

            let meta = fs::metadata(&link_path)
                .map_err(|_| SymlinkResolveError::CouldNotReadSymlink)?;
            if !meta.is_symlink() {
                return Ok(link_path)
            }
        }

        Err(SymlinkResolveError::MaxSymLinkDepthExceeded {
            max_depth: SYM_LINK_MAX_DEPTH,
        })
    }
}

pub enum SymlinkResolveError {
    SymLinkLoopDetected,
    MaxSymLinkDepthExceeded {
        max_depth: usize
    },
    CouldNotReadSymlink,
}

impl Display for SymlinkResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SymlinkResolveError::SymLinkLoopDetected => write!(f, "Loop detected"),
            SymlinkResolveError::MaxSymLinkDepthExceeded {
                max_depth
            } => write!(f, "Max supported symlink depth exceeded (max: {max_depth})"),
            SymlinkResolveError::CouldNotReadSymlink => write!(f, "Could not read symlink"),
        }
    }
}