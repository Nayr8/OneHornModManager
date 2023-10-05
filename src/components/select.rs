use std::ops::Index;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    #[prop_or_default]
    pub options: Rc<Vec<String>>,
    pub selected_profile: UseStateHandle<usize>,
}

#[function_component(Select)]
pub fn profile_select(props: &SelectProps) -> Html {
    let open = use_state(|| false);

    let on_open = {
        let open = open.clone();
        Callback::from(move |_| {
            open.set(!*open);
        })
    };

    let select_class = if *open {
        "select open"
    } else {
        "select"
    };

    let onselect = {
        let selected_profile = props.selected_profile.clone();
        Callback::from(move |value: usize| {
            selected_profile.set(value);
        })
    };

    html! {
        <div class="select-wrapper" onclick={on_open}>
            <div class={select_class}>
                <div class="select__trigger"><span>{props.options.index(*props.selected_profile).clone()}</span>
                    <div class="arrow"></div>
                </div>
                <div class="custom-options">
                    <SelectOptions options={props.options.clone()} selected={*props.selected_profile} onselect={onselect} />
                </div>
            </div>
        </div>
    }
}
#[derive(Properties, PartialEq)]
pub struct SelectOptionsProps {
    pub options: Rc<Vec<String>>,
    pub selected: usize,
    pub onselect: Callback<usize>,
}

#[function_component(SelectOptions)]
pub fn profile_select_options(props: &SelectOptionsProps) -> Html {
    props.options.iter().enumerate().map(|(index, option)| {
        if index == props.selected {
            html!(<span class="custom-option selected">{option}</span>)
        } else {
            let onselect = props.onselect.clone();
            html!(<span class="custom-option" onclick={move |_| onselect.emit(index)}>{option}</span>)
        }
    }).collect()
}

