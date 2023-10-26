use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;
use models::{FileEntry, Mod};
use crate::bindings::ModManager;
use crate::components::Spinner;
use crate::components::Button;

#[derive(Properties, PartialEq)]
pub struct AddNewModMenuProps {
    pub current_file: UseStateHandle<Option<FileEntry>>,
}
#[function_component(AddNewModMenu)]
pub fn add_new_mod_menu(props: &AddNewModMenuProps) -> Html {
    let details = use_state(|| None);
    let details_error = use_state(|| None);

    use_effect_with_deps(|(current_file, details, details_error)| {
        ModManager::get_mod_details(current_file.path.clone(), details.clone(), details_error.clone())
    }, (props.current_file.deref().clone().unwrap(), details.clone(), details_error.clone()));

    match details_error.as_ref() {
        Some(error) => html! {
            <div style="margin: auto;text-align: center">
                <div style="font-size: 2.5em">{"Error"}</div>
                <div style="margin-bottom: 2em">{format!("{error:?}")}</div>
                <div style="margin: auto;width: min-content;font-size: 1.5em"><Button>{"Back"}</Button></div>
            </div>
        },
        None => match details.as_ref() {
            Some(details) => html! {
            <div style="margin: auto;text-align: center">
                <div style="font-size: 2.5em">{format!("{}", details.name)}</div>
                <div style="margin-bottom: 2em">{format!("{}", details.description)}</div>
                <div style="margin: auto;width: min-content;font-size: 1.5em"><Button>{"Add mod"}</Button></div>
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

#[derive(Properties, PartialEq)]
pub struct AddNewModMenuLoadedProps {
    pub mod_details: Rc<Mod>,
}
#[function_component(AddNewModMenuLoaded)]
fn add_new_mod_menu_loaded(props: &AddNewModMenuLoadedProps) -> Html {
    html! {
        <div>{format!("{} -- {}", props.mod_details.name, props.mod_details.description)}</div>
    }
}