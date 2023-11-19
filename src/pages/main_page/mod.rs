use std::rc::Rc;
use yew::prelude::*;
use models::{Mod, Status};
use mod_list::ModList;
use selected_mod_panel::SelectedMod;
use apply_mods_panel::ApplyModsPanel;

mod mod_list;
mod selected_mod_panel;
mod apply_mods_panel;


#[derive(Properties, PartialEq)]
pub struct MainPageProps {
    pub mods: UseStateHandle<Status<Rc<Vec<Mod>>>>,
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub file_explorer_open: UseStateHandle<bool>,
    pub profile_open: UseStateHandle<bool>,
    pub profile_create_new: UseStateHandle<bool>,
}
#[function_component(MainPage)]
pub fn main_page(props: &MainPageProps) -> Html {
    html! {
        <div class="main-page">
            <ModList
                mods={props.mods.clone()}
                selected_mod={props.selected_mod.clone()}
                file_explorer_open={props.file_explorer_open.clone()}
                profile_open={props.profile_open.clone()}
                profile_create_new={props.profile_create_new.clone()} />
            <SelectedMod
                mods={props.mods.clone()}
                selected_mod={props.selected_mod.clone()} />
            <ApplyModsPanel />
        </div>
    }
}
