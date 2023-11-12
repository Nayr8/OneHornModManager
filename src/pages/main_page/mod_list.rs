use std::rc::Rc;
use yew::prelude::*;
use models::{Mod, Status};
use crate::bindings::ModManager;
use crate::components::Button;
use crate::components::button::ButtonSize;
use crate::components::Spinner;

#[derive(Properties, PartialEq)]
pub struct ModListProps {
    pub mods: UseStateHandle<Status<Rc<Vec<Mod>>>>,
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub file_explorer_open: UseStateHandle<bool>,
}
#[function_component(ModList)]
pub fn mod_list(props: &ModListProps) -> Html {
    use_effect_with_deps(|mods| {
        ModManager::get_mods(mods.clone());
    }, props.mods.clone());


    if let Status::Loaded(mods) = props.mods.as_ref() {
        let toggle_file_explorer = {
            let file_explorer_open = props.file_explorer_open.clone();
            let selected_mod = props.selected_mod.clone();
            move |_: MouseEvent| {
                file_explorer_open.set(!*file_explorer_open);
                selected_mod.set(None);
            }
        };
        if mods.len() == 0 {

            html! {
                <div style="margin: auto;text-align: center">
                    <div style="font-size: 2.5em">{"No Mods Found"}</div>
                    <Button onclick={toggle_file_explorer.clone()} size={ButtonSize::Big} style="margin: auto;margin-top: 1em;width: min-content">{"Add Mod"}</Button>
                </div>
            }
        } else {

            let mods_html: Html = mods.iter().enumerate().map(|(index, mod_info)| html! {
                <ModElement mod_info={(*mod_info).clone()} selected_mod={props.selected_mod.clone()} index={index} />
            }).collect();

            html! {
                <div class="mod-list">
                    { mods_html }
                    <Button onclick={toggle_file_explorer.clone()} size={ButtonSize::Big} style="margin: auto;width: min-content;margin-top: 3em;margin-bottom: 3em">{"Add Mod"}</Button>
                </div>
            }
        }
    } else {
        html! {
            <Spinner />
        }
    }

}

#[derive(Properties, PartialEq)]
pub struct ModElementProps {
    pub mod_info: Mod,
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub index: usize,
}
#[function_component(ModElement)]
fn mod_component(props: &ModElementProps) -> Html {
    let selected = if let Some(selected_index) = *props.selected_mod {
        selected_index == props.index
    } else {
        false
    };

    let onclick = if selected {
        let selected_mod = props.selected_mod.clone();
        Callback::from(move |_: MouseEvent| {
            selected_mod.set(None);
        })
    } else {
        let selected_mod = props.selected_mod.clone();
        let index = props.index;
        Callback::from(move |_: MouseEvent| {
            selected_mod.set(Some(index));
        })
    };

    html! {

        <Button class={classes!("mod-element", if props.mod_info.enabled { None } else { Some("make-element-disabled-lite") })} size={ButtonSize::Thin} onclick={onclick} selected={selected}>
            <div style="font-size: 0.9em;">{props.mod_info.name.clone()}</div>
            <div style="font-size: 0.7em;transform: translate(0, 0.2em);overflow-x:hidden;text-overflow: ellipsis">{props.mod_info.description.clone()}</div>
            <div style="font-size: 0.9em">{props.mod_info.version.clone()}</div>
        </Button>
    }
}