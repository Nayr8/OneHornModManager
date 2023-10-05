use std::rc::Rc;
use yew::prelude::*;
use models::Mod;

#[derive(Properties, PartialEq)]
pub struct ModListProps {
    pub mods: Rc<Vec<Mod>>,
    pub selected_mod: UseStateHandle<Option<usize>>,
}
#[function_component(ModList)]
pub fn mod_list(props: &ModListProps) -> Html {

    let mods_html: Html = props.mods.iter().enumerate().map(|(index, mod_info)| html! {
        <ModElement mod_info={(*mod_info).clone()} selected_mod={props.selected_mod.clone()} index={index} />
    }).collect();

    html! {
        <div class="mod-list">
            { mods_html }
        </div>
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

    if let Some(selected_index) = *props.selected_mod {
        if selected_index == props.index {
            let onclick = {
                let selected_mod = props.selected_mod.clone();
                move |_| {
                    selected_mod.set(None);
                }
            };

            return html! {
                <div class="element element-button element-selected mod-element" onclick={onclick}>
                    <div>{props.mod_info.name.clone()}</div>
                    <div>{props.mod_info.description.clone()}</div>
                </div>
            }
        }
    }
    let onclick = {
        let selected_mod = props.selected_mod.clone();
        let index = props.index;
        move |_| {
            selected_mod.set(Some(index))
        }
    };

    html! {
        <div class="element element-button mod-element" onclick={onclick}>
            <div>{props.mod_info.name.clone()}</div>
            <div>{props.mod_info.description.clone()}</div>
        </div>
    }
}