use std::path::PathBuf;
use std::rc::Rc;
use yew::prelude::*;
use models::FileEntry;
use crate::bindings::FileBrowser;
use location::FileExplorerLocation;
use files::Files;
use current_file::CurrentFile;
use file_nav::FileNav;

mod location;
mod files;
mod current_file;
mod file_nav;

#[derive(Properties, PartialEq)]
pub struct FileExplorerProps {
    pub file_explorer_open: UseStateHandle<bool>,
}
#[function_component(FileExplorer)]
pub fn file_explorer(_props: &FileExplorerProps) -> Html {
    let current_path = use_state(|| Rc::new(PathBuf::new()));
    let current_entries = use_state(|| Vec::<FileEntry>::new());

    let current_file: UseStateHandle<Option<FileEntry>> = use_state(|| None);

    use_effect_with_deps(|(current_path, current_entries)| {
        FileBrowser::read_current_dir_into((*current_path).clone(), (*current_entries).clone());
    }, (current_path.clone(), current_entries.clone()).clone());

    let current_directory_str = current_path.to_string_lossy().to_string();

    html! {
        <div class="file-explorer">
            <FileNav />
            <FileExplorerLocation current_directory={current_directory_str} />
            <Files
                current_path={current_path.clone()}
                current_entries={current_entries.clone()}
                current_file={current_file.clone()} />
            <CurrentFile current_file={current_file.clone()} />
        </div>
    }
}
