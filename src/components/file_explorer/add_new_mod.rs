use yew::prelude::*;
use models::FileEntry;

#[derive(Properties, PartialEq)]
pub struct AddNewModMenuProps {
    pub current_file: UseStateHandle<Option<FileEntry>>,
}
#[function_component(AddNewModMenu)]
pub fn add_new_mod_menu(props: &AddNewModMenuProps) -> Html {
    html! {
        <div>{format!("{:?}", (*props.current_file).as_ref().unwrap())}</div>
    }
}