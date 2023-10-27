use std::ops::Deref;
use yew::prelude::*;
use models::FileEntry;
use crate::bindings::ModManager;
use crate::components::Spinner;
use crate::components::Button;

#[derive(Properties, PartialEq)]
pub struct AddNewModMenuProps {
    pub current_file: UseStateHandle<Option<FileEntry>>,
    pub add_mod_menu: UseStateHandle<bool>,
    pub file_explorer_open: UseStateHandle<bool>,
}
#[function_component(AddNewModMenu)]
pub fn add_new_mod_menu(props: &AddNewModMenuProps) -> Html {
    let details = use_state(|| None);
    let details_error = use_state(|| None);

    use_effect_with_deps(|(current_file, details, details_error)| {
        ModManager::get_mod_details(current_file.path.clone(), details.clone(), details_error.clone())
    }, (props.current_file.deref().clone().unwrap(), details.clone(), details_error.clone()));

    let close_mod_menu = {
        let current_file = props.current_file.clone();
        let add_mod_menu = props.add_mod_menu.clone();
        move |_: MouseEvent| {
            current_file.set(None);
            add_mod_menu.set(false);
        }
    };

    let add_mod = {
        let file_explorer_open = props.file_explorer_open.clone();
        move |_: MouseEvent| {
            ModManager::add_mod();
            file_explorer_open.set(false);
        }
    };

    match details_error.as_ref() {
        Some(error) => html! {
            <div style="margin: auto;text-align: center">
                <div style="font-size: 2.5em">{"Error"}</div>
                <div style="margin-bottom: 2em">{format!("{error:?}")}</div>
                <Button onclick={close_mod_menu} style="margin: auto;width: min-content;font-size: 1.5em">{"Back"}</Button>
            </div>
        },
        None => match details.as_ref() {
            Some(details) => html! {
            <div style="margin: auto;text-align: center">
                <div style="font-size: 2.5em">{format!("{}", details.name)}</div>
                <div>{format!("{}", details.description)}</div>
                <div style="font-size: 1.5em;margin-top: 2em;display: flex;justify-content: center">
                    <Button onclick={close_mod_menu} style="width: min-content">{"Back"}</Button>
                    <Button onclick={add_mod} style="width: min-content">{"Add mod"}</Button>
                </div>
            </div>
            },
            None => html! {
                <div style="margin: auto">
                    <Spinner />
                    <div>{"Reading Metadata..."}</div>
                </div>
            }
        }
    }
}