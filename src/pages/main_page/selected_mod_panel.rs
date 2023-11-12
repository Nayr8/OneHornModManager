use std::rc::Rc;
use yew::prelude::*;
use models::{Mod, Status};
use crate::bindings::ModManager;
use crate::components::Button;
use crate::components::button::ButtonSize;

#[derive(Properties, PartialEq)]
pub struct SelectedModProps {
    pub mods: UseStateHandle<Status<Rc<Vec<Mod>>>>,
    pub selected_mod: UseStateHandle<Option<usize>>,
}
#[function_component(SelectedMod)]
pub fn selected_mod(props: &SelectedModProps) -> Html {
    let selected_mod = (|selected_mod, mods| {
        let selected_mod: &usize = selected_mod?;
        let Status::Loaded(mods): Status<&Rc<Vec<Mod>>, &()> = mods else { return None };
        mods.get(*selected_mod)
    })(props.selected_mod.as_ref(), props.mods.as_ref());


    if let Some(selected_mod) = selected_mod {
        let toggle_enabled = {
            let enabled = selected_mod.enabled;
            let mod_index = props.selected_mod.unwrap();
            let mods = props.mods.clone();
            move |_: MouseEvent| {
                ModManager::set_mod_enabled_state(mod_index, !enabled);
                ModManager::get_mods(mods.clone());
            }
        };

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
            <div class="selected-mod-panel">
                <div style="font-size: 1.3em;text-align: center">{&selected_mod.name}</div>
                <div style="font-size: 0.8em;text-align: center">{&selected_mod.description}</div>
                <div style="font-size: 1em;text-align: center">{&selected_mod.version}</div>
                <div class="selected-mod-options">
                    <Button onclick={remove_mod} size={ButtonSize::Big} style="width: min-content">{"Remove Mod"}</Button>
                    <Button onclick={toggle_enabled} size={ButtonSize::Big} style="width: min-content">
                        if selected_mod.enabled {
                            {"Disable Mod"}
                        } else {
                            {"Enable Mod"}
                        }
                    </Button>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="no-selected-mod-panel">
                {"Select a mod for more options"}
            </div>
        }
    }
}
