use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SvgProps {
    pub svg_path: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub override_colour: Option<String>,
}

#[function_component]
pub fn Svg(props: &SvgProps) -> Html {
    match props.override_colour.as_ref() {
        Some(colour) => html! {
            <div
                style={format!("\
                    mask: url('{}') no-repeat 100% 100%;\
                    mask-size: cover;\
                    background-color: {};{}\
                ", props.svg_path, colour, props.style)}
                class={classes!(props.class.clone())}>
            </div>
        },
        None => html! {
            <div
                style={format!("\
                    background-image: url('{}');\
                    background-size: 100% 100%;{}\
                    ", props.svg_path, props.style)}
                class={classes!(props.class.clone())}>
            </div>
        },
    }
}