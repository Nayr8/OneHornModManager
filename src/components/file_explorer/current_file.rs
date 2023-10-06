use std::ops::Deref;
use std::path::PathBuf;
use yew::prelude::*;
use crate::components::{Button, Element};


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
                <Element class="current-file">{&current_file.name}</Element>
                <Button class="current-file-confirm">{"Confirm"}</Button>
            } else {
                <Element class="current-file"></Element>
                <Button class="current-file-confirm" disabled=true>{"Confirm"}</Button>
            }
        </div>
    }
}