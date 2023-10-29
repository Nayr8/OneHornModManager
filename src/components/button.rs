use yew::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonSize {
    Thin, Standard, Big
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Standard
    }
}

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
    pub size: ButtonSize,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = classes!(
        "element",
        if props.disabled { "make-element-disabled" } else { "make-element-button" },
        match props.size {
            ButtonSize::Thin => Some("make_element-thin"),
            ButtonSize::Standard => None,
            ButtonSize::Big => Some("make_element-big"),
        },
        if props.selected { Some("make-element-selected") } else { None },
        props.class.clone()
    );

    if let Some(onclick) = &props.onclick {
        if !props.disabled {
            return html! {
                <div class={classes} style={props.style.clone()} onclick={onclick}>
                    {props.children.clone()}
                </div>
            };
        }
    }

    html! {
        <div class={classes} style={props.style.clone()}>
            {props.children.clone()}
        </div>
    }
}