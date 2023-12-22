use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub highlight_on_hover: bool,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let onclick = if props.disabled {
        None
    } else {
        Some(props.onclick.clone())
    };

    let classes = if props.disabled {
        props.class.clone()
    } else {
        classes!("button", props.class.clone(), props.highlight_on_hover.then_some("highlight_on_hover"))
    };
    html! {
        <div class={classes} onclick={onclick} style={props.style.clone()}>
            {props.children.clone()}
        </div>
    }
}