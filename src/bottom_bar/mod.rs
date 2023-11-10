use yew::prelude::*;
use crate::components::Button;

#[derive(Properties, PartialEq)]
pub struct BottomBarProps {
    pub console_open: UseStateHandle<bool>,
}

#[function_component(BottomBar)]
pub fn bottom_bar(props: &BottomBarProps) -> Html {
    let toggle_console = {
        let console_open = props.console_open.clone();
        move |_: MouseEvent| {
            console_open.set(!*console_open);
        }
    };

    html! {
        <div class="bottom-bar">
            <Button onclick={toggle_console}>{"Toggle Console"}</Button>
        </div>
    }
}