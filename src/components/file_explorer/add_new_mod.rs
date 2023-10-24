use std::ops::Deref;
use yew::prelude::*;
use models::FileEntry;
use crate::bindings::ModManager;

#[derive(Properties, PartialEq)]
pub struct AddNewModMenuProps {
    pub current_file: UseStateHandle<Option<FileEntry>>,
}
#[function_component(AddNewModMenu)]
pub fn add_new_mod_menu(props: &AddNewModMenuProps) -> Html {
    let details = use_state(|| None);

    use_effect_with_deps(|(current_file, details)| {
        ModManager::get_mod_details(current_file.path.clone(), details.clone())
    }, (props.current_file.deref().clone().unwrap(), details.clone()));

    html! {
        <div>
            <div>{format!("{:?}", (*props.current_file).as_ref().unwrap())}</div>
            if let Some(details) = details.as_ref() {
                <div>{format!("A: {} {}", details.name, details.description)}</div>
            }
        </div>
    }
}