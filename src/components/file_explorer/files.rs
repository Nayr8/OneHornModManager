use std::path::PathBuf;
use yew::prelude::*;
use models::{EntryType, FileEntry};
use crate::bindings::FileBrowser;
use crate::components::file_explorer::current_file::FileInfo;


#[derive(Properties, PartialEq)]
pub struct FilesProps {
    pub current_path: UseStateHandle<PathBuf>,
    pub current_entries: UseStateHandle<Vec<FileEntry>>,
    pub current_file: UseStateHandle<Option<FileInfo>>,
}

#[function_component(Files)]
pub fn files(props: &FilesProps) -> Html {

    let entries_html: Html = props.current_entries.iter().filter(|entry| !entry.file_name.starts_with(".")).map(|entry|  {
        html! {
            <DirectoryEntry
                entry={(*entry).clone()}
                current_path={props.current_path.clone()}
                current_entries={props.current_entries.clone()}
                current_file={props.current_file.clone()}  />
        }
    }).collect();

    let parent = props.current_path.parent();
    html! {
        <div class="files">
            if let Some(parent) = parent {
                <DirectoryParent
                parent={parent.to_owned()}
                current_path={props.current_path.clone()}
                current_entries={props.current_entries.clone()} />
            }
            { entries_html }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DirectoryParentProps {
    parent: PathBuf,
    current_path: UseStateHandle<PathBuf>,
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
        <div class="element element-button file dir" onclick={onclick}>
            <div />
            <div>{".."}</div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DirectoryEntryProps {
    entry: FileEntry,
    current_path: UseStateHandle<PathBuf>,
    current_entries: UseStateHandle<Vec<FileEntry>>,
    current_file: UseStateHandle<Option<FileInfo>>,
}

#[function_component(DirectoryEntry)]
pub fn directory_entry(props: &DirectoryEntryProps) -> Html {
    let navigate_to = {
        let path = props.entry.path.clone();
        let current_path = props.current_path.clone();
        let current_entries = props.current_entries.clone();
        move |_: MouseEvent| {
            FileBrowser::redirect_browser(path.clone());
            FileBrowser::read_current_dir_into(current_path.clone(), current_entries.clone());
        }
    };

    let is_selected = if let Some(selected_file) = props.current_file.as_ref() {
        selected_file.path == props.entry.path
    } else {
        false
    };

    let select_file = {
        let file_info = FileInfo {
            name: props.entry.file_name.clone(),
            path: props.entry.path.clone(),
        };
        let current_file = props.current_file.clone();
        move |_:MouseEvent| {
            if is_selected {
                current_file.set(None);
            } else {
                current_file.set(Some(file_info.clone()));
            }
        }
    };

    match props.entry.entry_type {
        EntryType::File => if is_selected {
            html! {
                <div class="element element-button-thin-selected file" onclick={select_file}>
                    <div>{"FILE"}</div>
                    <div>
                        { &props.entry.file_name }
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="element element-button-thin file" onclick={select_file}>
                    <div>{"FILE"}</div>
                    <div>
                        { &props.entry.file_name }
                    </div>
                </div>
            }
        },
        EntryType::Directory => html! {
            <div class="element element-button-thin file dir" onclick={navigate_to}>
                <div>{"DIR"}</div>
                <div>
                    { &props.entry.file_name }
                </div>
            </div>
        },
    }
}