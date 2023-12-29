use yew::prelude::*;
use crate::components::Svg;

#[derive(Properties, PartialEq)]
pub struct ErrorMessageProps {
    pub message: String,
}

#[function_component]
pub fn ErrorMessage(props: &ErrorMessageProps) -> Html {
    html! {
        <div class="error">
            <Svg svg_path="public/images/error.svg" width=5.0 height=5.0/>
            <div style="font-size: 1.5em">{&props.message}</div>
        </div>
    }
}
