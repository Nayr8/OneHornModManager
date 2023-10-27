use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub thin: bool,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = match (props.selected, props.disabled, props.thin) {
        (false, false, false) => "element-button",
        (false, false, true) => "element-button-thin",
        (false, true, false) => "element-disabled",
        (false, true, true) => "element-disabled-thin",
        (true, false, false) => "element-button-selected",
        (true, false, true) => "element-button-thin-selected",
        (true, true, false) => "element-disabled",
        (true, true, true) => "element-disabled-thin-selected",
    };

    if let Some(onclick) = &props.onclick {
        if !props.disabled {
            return html! {
                <div class={classes!(class, props.class.clone())} onclick={onclick}>
                    {props.children.clone()}
                </div>
            };
        }
    }

    html! {
        <div class={classes!(class, props.class.clone())} style={props.style.clone()}>
            {props.children.clone()}
        </div>
    }
}