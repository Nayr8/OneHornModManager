use yew::prelude::*;

#[derive(PartialEq, Copy, Clone, Default)]
pub enum SpinnerSize {
    Small,
    #[default]
    Normal,
}
#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub size: SpinnerSize,
}

#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let spinner_class = match props.size {
        SpinnerSize::Small => "small-spinner",
        SpinnerSize::Normal => "spinner",
    };
    html! {
        <div class={classes!(spinner_class, props.class.clone())} style={props.style.clone()} />
    }
}