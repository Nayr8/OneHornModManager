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
}

#[function_component(TopBar)]
pub fn top_bar(props: &TopBarProps) -> Html {
    html! {
        <div class="top-bar">
            if *props.file_explorer_open {
                <Profiles selected_mod={props.selected_mod.clone()} mods={props.mods.clone()} disabled=true />
            } else {
                <Profiles selected_mod={props.selected_mod.clone()} mods={props.mods.clone()} />
            }
        </div>
    }
}