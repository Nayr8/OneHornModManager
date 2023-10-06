use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ElementProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub thin: bool,
}

#[function_component(Element)]
pub fn element(props: &ElementProps) -> Html {
    let class = match props.thin {
        true => "element-thin",
        false => "element",
    };

    html! {
        <div class={classes!(class, props.class.clone())}>
            {props.children.clone()}
        </div>
    }
}