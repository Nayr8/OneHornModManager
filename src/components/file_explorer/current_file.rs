use std::ops::Deref;
use std::path::PathBuf;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Properties, PartialEq)]
pub struct CurrentFileProps {
    pub current_file: UseStateHandle<Option<FileInfo>>,
}

#[function_component(CurrentFile)]
pub fn files(props: &CurrentFileProps) -> Html {
    html! {
        <div class="file-base">
            if let Some(current_file) = props.current_file.deref() {
                <div class="current-file element">{&current_file.name}</div>
                <div class="current-file-confirm element-button">{"Confirm"}</div>
            } else {
                <div class="current-file element"></div>
                <div class="current-file-confirm element-disabled">{"Confirm"}</div>
            }
        </div>
    }
}