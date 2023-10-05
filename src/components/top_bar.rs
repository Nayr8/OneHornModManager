use yew::prelude::*;

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
        move |_| {
            file_explorer_open.set(!*file_explorer_open);
            selected_mod.set(None);
        }
    };
    html! {
        <div class="top-bar">
            if *props.file_explorer_open {
                <div class="element element-button" onclick={toggle_file_explorer}>{"Back to Mod List"}</div>
            } else {
                <div class="element element-button" onclick={toggle_file_explorer}>{"Add Mod"}</div>

                if let Some(_mod_index) = *props.selected_mod {
                    <div class="element element-button">{"Remove Mod"}</div>
                } else {
                    <div class="element element-disable">{"Remove Mod"}</div>
                }
            }
        </div>
    }
}