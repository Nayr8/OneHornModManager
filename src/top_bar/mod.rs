use std::rc::Rc;
use yew::prelude::*;
use models::{Mod, Status};
use crate::bindings::ModManager;
use crate::components::Button;
use crate::components::Spinner;
use crate::components::spinner::SpinnerSize;
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
    let toggle_file_explorer = {
        let file_explorer_open = props.file_explorer_open.clone();
        let selected_mod = props.selected_mod.clone();
        move |_: MouseEvent| {
            file_explorer_open.set(!*file_explorer_open);
            selected_mod.set(None);
        }
    };

    let applying = use_state(|| false);

    let apply = {
        let applying = applying.clone();
        move |_: MouseEvent| {
            ModManager::apply(applying.clone());
        }
    };

    html! {
        <div class="top-bar">
            if *props.file_explorer_open {
                <Button onclick={toggle_file_explorer.clone()}>
                    {"Back to Mod List"}
                </Button>
            } else {
                <Profiles selected_mod={props.selected_mod.clone()} mods={props.mods.clone()} />
                <Button onclick={toggle_file_explorer.clone()}>
                    {"Add Mod"}
                </Button>
                <Button onclick={apply} disabled={*applying}>
                    if *applying {
                        <Spinner size={SpinnerSize::Small} />
                    } else {
                        {"Apply"}
                    }
                </Button>
            }
        </div>
    }
}