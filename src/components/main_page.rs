use std::rc::Rc;
use yew::prelude::*;
use models::Mod;
use crate::bindings::ModManager;
use crate::components::mod_list::ModList;
use crate::components::Button;
use crate::components::button::ButtonSize;


#[derive(Properties, PartialEq)]
pub struct MainPageProps {
    pub mods: UseStateHandle<Option<Rc<Vec<Mod>>>>,
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub file_explorer_open: UseStateHandle<bool>,
}
#[function_component(MainPage)]
pub fn main_page(props: &MainPageProps) -> Html {
    html! {
        <div class="main-page">
            <ModList
                mods={props.mods.clone()}
                selected_mod={props.selected_mod.clone()}
                file_explorer_open={props.file_explorer_open.clone()} />
            <SelectedMod
                mods={props.mods.clone()}
                selected_mod={props.selected_mod.clone()} />
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct SelectedModProps {
    pub mods: UseStateHandle<Option<Rc<Vec<Mod>>>>,
    pub selected_mod: UseStateHandle<Option<usize>>,
}
#[function_component(SelectedMod)]
pub fn selected_mod(props: &SelectedModProps) -> Html {
    let selected_mod = (|selected_mod, mods| {
        let selected_mod: &usize = selected_mod?;
        let mods: &Rc<Vec<Mod>> = mods?;
        mods.get(*selected_mod)
    })(props.selected_mod.as_ref(), props.mods.as_ref());


    if let Some(selected_mod) = selected_mod {
        let remove_mod = {
            let mods = props.mods.clone();
            let selected_mod = props.selected_mod.clone();
            let selected_mod_index = props.selected_mod.unwrap();
            move |_: MouseEvent| {
                ModManager::remove_mod(selected_mod_index, mods.clone());
                selected_mod.set(None);
            }
        };

        html! {
            <div class="selected-mod">
                <div style="font-size: 1.3em;text-align: center">{&selected_mod.name}</div>
                <div style="font-size: 0.8em;text-align: center">{&selected_mod.description}</div>
                <div style="font-size: 1em;text-align: center">{&selected_mod.version}</div>
                <div class="selected-mod-options">
                    <Button onclick={remove_mod} size={ButtonSize::Big} style="width: min-content">{"Remove Mod"}</Button>
                    <Button size={ButtonSize::Big} style="width: min-content">{"Disable Mod"}</Button>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="no-selected-mod">
                {"Select a mod for more options"}
            </div>
        }
    }
}