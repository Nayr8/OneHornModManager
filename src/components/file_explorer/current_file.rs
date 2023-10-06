use std::ops::Deref;
use yew::prelude::*;
use models::FileEntry;
use crate::components::{Button, Element};
#[derive(Properties, PartialEq)]
pub struct CurrentFileProps {
    pub current_file: UseStateHandle<Option<FileEntry>>,
}

#[function_component(CurrentFile)]
pub fn files(props: &CurrentFileProps) -> Html {


    html! {
        <div class="file-base">
            if let Some(current_file) = props.current_file.deref() {
                <Element class="current-file">{&current_file.file_name}</Element>
                <Button class="current-file-confirm">{"Confirm"}</Button>
            } else {
                <Element class="current-file"></Element>
                <Button class="current-file-confirm" disabled=true>{"Confirm"}</Button>
            }
        </div>
    }
}