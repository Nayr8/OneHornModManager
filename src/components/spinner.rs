use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub class: Classes,
    pub size: f32,
}


#[function_component]
pub fn Spinner(props: &SpinnerProps) -> Html {
    let style = format!("width: {0}em;height: {0}em;border-width: {1}em;{2}", props.size, props.size / 15.0, props.style);
    html! {
        <div class={classes!("spinner", props.class.clone())} style={style}/>
    }
}
