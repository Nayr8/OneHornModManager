use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BottomBarProps {
}

#[function_component(BottomBar)]
pub fn bottom_bar(_props: &BottomBarProps) -> Html {
    html! {
        <div class="bottom-bar">
        </div>
    }
}