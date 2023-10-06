use std::rc::Rc;
use yew::prelude::*;
use models::Mod;
use crate::components::Button;

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
            selected_mod.set(Some(index))
        })
    };

    html! {
        <Button class="mod-element" onclick={onclick} selected={selected}>
            <div>{props.mod_info.name.clone()}</div>
            <div>{props.mod_info.description.clone()}</div>
        </Button>
    }
}