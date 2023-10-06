use yew::prelude::*;
use crate::components::Button;

#[derive(Properties, PartialEq)]
pub struct TopBarProps {
    pub file_explorer_open: UseStateHandle<bool>,
    pub selected_mod: UseStateHandle<Option<usize>>,
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

                if let Some(_mod_index) = *props.selected_mod {
                    <Button>
                        {"Remove Mod"}
                    </Button>
                } else {
                    <Button disabled=true>
                        {"Remove Mod"}
                    </Button>
                }
            }
        </div>
    }
}