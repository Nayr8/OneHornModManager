use std::rc::Rc;
use yew::prelude::*;
use models::{Mod, Status};
use profiles::Profiles;

mod profiles;

#[derive(Properties, PartialEq)]
pub struct TopBarProps {
    pub file_explorer_open: UseStateHandle<bool>,
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub mods: UseStateHandle<Status<Rc<Vec<Mod>>>>,
    pub profile_open: UseStateHandle<bool>,
    pub profile_create_new: UseStateHandle<bool>,
}

#[function_component(TopBar)]
pub fn top_bar(props: &TopBarProps) -> Html {
    html! {
        <div class="top-bar">
            <Profiles selected_mod={props.selected_mod.clone()}
                mods={props.mods.clone()}
                profile_open={props.profile_open.clone()}
                profile_create_new={props.profile_create_new.clone()}
                disabled={*props.file_explorer_open} />
        </div>
    }
}