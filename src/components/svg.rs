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
    pub width: f32,
    pub height: f32,
    #[prop_or_default]
    pub flip_x: bool,
    #[prop_or_default]
    pub flip_y: bool,
}

#[function_component]
pub fn Svg(props: &SvgProps) -> Html {
    let transform = match (props.flip_x, props.flip_y) {
        (false, false) => "",
        (true, false) => "transform: scaleX(-1);",
        (false, true) => "transform: scaleY(-1);",
        (true, true) => "transform: scale(-1);",
    };

    let shared_style = format!("\
        width: {}em;\
        height: {}em;\
        {}\
    ", props.width, props.height, transform);

    match props.override_colour.as_ref() {
        Some(colour) => html! {
            <div
                style={format!("\
                    mask: url('{}') no-repeat 100% 100%;\
                    mask-size: cover;\
                    background-color: {};{}{}\
                ", props.svg_path, colour, shared_style, props.style)}
                class={classes!(props.class.clone())}>
            </div>
        },
        None => html! {
            <div
                style={format!("\
                    background-image: url('{}');\
                    background-size: 100% 100%;{}{}\
                ", props.svg_path, shared_style, props.style)}
                class={classes!(props.class.clone())}>
            </div>
        },
    }
}