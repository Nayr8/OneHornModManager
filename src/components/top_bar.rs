use std::rc::Rc;
use yew::prelude::*;
use models::Mod;
use crate::bindings::ModManager;
use crate::components::Button;

#[derive(Properties, PartialEq)]
pub struct TopBarProps {
    pub file_explorer_open: UseStateHandle<bool>,
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub mods: UseStateHandle<Option<Rc<Vec<Mod>>>>,
}

#[function_component(TopBar)]
pub fn top_bar(props: &TopBarProps) -> Html {
    let toggle_file_explorer = {
        let file_explorer_open = props.file_explorer_open.clone();
        let selected_mod = props.selected_mod.clone();
        move |_: MouseEvent| {
            file_explorer_open.set(!*file_explorer_open);
            selected_mod.set(None);
        }
    };

    let remove_mod = props.selected_mod.map(|mod_index| {
        let mods = props.mods.clone();
        move |_| {
            ModManager::remove_mod(mod_index, mods.clone());
        }
    });

    let save = |_: MouseEvent| {
        ModManager::save();
    };

    let apply = |_: MouseEvent| {
        ModManager::apply();
    };

    html! {
        <div class="top-bar">
            if *props.file_explorer_open {
                <Button onclick={toggle_file_explorer.clone()}>
                    {"Back to Mod List"}
                </Button>
            } else {
                <Button onclick={toggle_file_explorer.clone()}>
                    {"Add Mod"}
                </Button>

                <Button disabled={remove_mod.is_none()} onclick={remove_mod}>
                    {"Remove Mod"}
                </Button>

                <Button onclick={save}>
                    {"Save"}
                </Button>

                <Button onclick={apply}>
                    {"Apply"}
                </Button>
            }
        </div>
    }
}