use std::path::PathBuf;
use std::sync::Arc;
use yew::prelude::*;
use models::{EntryType, FileEntry};
use crate::bindings::FileBrowser;
use crate::components::Button;
use crate::components::button::ButtonSize;


#[derive(Properties, PartialEq)]
pub struct FilesProps {
    pub current_path: UseStateHandle<Arc<PathBuf>>,
    pub current_entries: UseStateHandle<Vec<FileEntry>>,
    pub current_file: UseStateHandle<Option<FileEntry>>,
    pub navigation_enabled_state: UseStateHandle<(bool, bool)>,
}

#[function_component(Files)]
pub fn files(props: &FilesProps) -> Html {

    let entries_html: Html = props.current_entries.iter().filter(|entry| !entry.file_name.starts_with(".")).map(|entry|  {
        html! {
            <DirectoryEntry
                entry={(*entry).clone()}
                current_path={props.current_path.clone()}
                current_entries={props.current_entries.clone()}
                current_file={props.current_file.clone()} 
                navigation_enabled_state={props.navigation_enabled_state.clone()} />
        }
    }).collect();

    let parent = props.current_path.parent();
    html! {
        <div class="files">
            if let Some(parent) = parent {
                <DirectoryParent
                parent={Arc::new(parent.to_owned())}
                current_path={props.current_path.clone()}
                current_entries={props.current_entries.clone()} />
            }
            { entries_html }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DirectoryParentProps {
    parent: Arc<PathBuf>,
    current_path: UseStateHandle<Arc<PathBuf>>,
    current_entries: UseStateHandle<Vec<FileEntry>>,
}

#[function_component(DirectoryParent)]
pub fn directory_parent(props: &DirectoryParentProps) -> Html {
    let onclick = {
        let parent = props.parent.clone();
        let current_path = props.current_path.clone();
        let current_entries = props.current_entries.clone();
        move |_: MouseEvent| {
            FileBrowser::redirect_browser(parent.clone());
            FileBrowser::read_current_dir_into(current_path.clone(), current_entries.clone());
        }
    };

    html! {
        <Button class="dir" onclick={onclick}>
            <img src="public/file_browser_up.png" style="height: 1.6em" />
            <div>{".."}</div>
        </Button>
    }
}

#[derive(Properties, PartialEq)]
pub struct DirectoryEntryProps {
    pub entry: FileEntry,
    pub current_path: UseStateHandle<Arc<PathBuf>>,
    pub current_entries: UseStateHandle<Vec<FileEntry>>,
    pub current_file: UseStateHandle<Option<FileEntry>>,
    pub navigation_enabled_state: UseStateHandle<(bool, bool)>,
}

#[function_component(DirectoryEntry)]
pub fn directory_entry(props: &DirectoryEntryProps) -> Html {
    let is_selected = if let Some(selected_file) = props.current_file.as_ref() {
        selected_file.path == props.entry.path
    } else {
        false
    };

    let (type_path, onclick, class): (&str, Callback<MouseEvent>, &str) = match props.entry.entry_type {
        EntryType::File => {
            let file_info = props.entry.clone();
            let current_file = props.current_file.clone();
            let icon_path = if props.entry.file_name.ends_with(".zip") {
                "public/file_browser_zip.png"
            } else {
                "public/file_browser_d20.png"
            };
            (icon_path, Callback::from(move |_:MouseEvent| {
                if is_selected {
                    current_file.set(None);
                } else {
                    current_file.set(Some(file_info.clone()));
                }
            }), "file")
        }
        EntryType::Directory => {
            let path = props.entry.path.clone();
            let current_path = props.current_path.clone();
            let current_entries = props.current_entries.clone();
            let navigation_enabled_state = props.navigation_enabled_state.clone();
            ("public/file_browser_folder.png", Callback::from(move |_: MouseEvent| {
                FileBrowser::redirect_browser(path.clone());
                FileBrowser::read_current_dir_into(current_path.clone(), current_entries.clone());
                FileBrowser::get_navigation_enabled_state(navigation_enabled_state.clone());
            }), "dir")
        }
    };

    html! {
        <Button class={classes!(class)} size={ButtonSize::Thin} selected={is_selected} onclick={onclick}>
            <img src={type_path} style="height: 1.6em" />
            <div>
                { &props.entry.file_name }
            </div>
        </Button>
    }
}