use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub tooltip: String,
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    if props.disabled {
        return html! { <>{props.children.clone()}</> }
    }

    html! {
        <div class="tooltip-wrapper">
            {props.children.clone()}
            <div class="tooltip-content">
                {&props.tooltip}
            </div>
        </div>
    }
}