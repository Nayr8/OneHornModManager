use yew::prelude::*;
use models::FileEntry;
use crate::components::{Button, Element};
#[derive(Properties, PartialEq)]
pub struct CurrentFileProps {
    pub current_file: UseStateHandle<Option<FileEntry>>,
    pub add_mod_menu: UseStateHandle<bool>,
}

#[function_component(CurrentFile)]
pub fn files(props: &CurrentFileProps) -> Html {
    let add_mod = {
        let add_mod_menu = props.add_mod_menu.clone();
        move |_: MouseEvent| {
            add_mod_menu.set(true);
        }
    };

    html! {
        <div class="file-base">
            if let Some(current_file) = &*props.current_file {
                <Element class="current-file">{&current_file.file_name}</Element>
                <Button class="current-file-confirm" onclick={add_mod}>{"Confirm"}</Button>
            } else {
                <Element class="current-file"></Element>
                <Button class="current-file-confirm" disabled=true>{"Confirm"}</Button>
            }
        </div>
    }
}