use yew::prelude::*;
use crate::bindings::ManagerBindings;
use crate::components::{Button, ErrorMessage, Spinner, Svg};
use crate::helpers::localisation::LocalisationHelper;
use crate::models::Mod;
use crate::Status;

#[derive(Properties, PartialEq)]
pub struct ModListProps {
    pub t: UseStateHandle<LocalisationHelper>,
}

#[function_component]
pub fn ModList(props: &ModListProps) -> Html {
    let mods = use_state(|| Status::Loading);

    use_effect_with_deps({
        let mods = mods.clone();
        move |_| {
            ManagerBindings::get_mods(mods);
        }
    }, ());

    match &*mods {
        Status::Loading => html! {
            <Spinner size=10.0 style="margin-left: auto;margin-right: auto;margin-top: 10%"/>
        },
        Status::Loaded(mods) => html! {
            <div class="mod-list">
                {mods.iter().enumerate().map(|(index, mod_data)| html! {
                    <ModRow
                        mod_data={mod_data.clone()}
                        index={index}
                    />
                }).collect::<Html>()}
            </div>
        },
        Status::Error(_) => html! {
            <ErrorMessage message={props.t.trans("page:mod_list:error_getting_mods")}/>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ModRowProps {
    pub mod_data: Mod,
    pub index: usize,
}

#[function_component]
pub fn ModRow(props: &ModRowProps) -> Html {
    html! {
        <div class="mod">
            <Button>
                if props.mod_data.enabled {
                    <Svg svg_path="public/images/switch.svg" class="state-switch-enabled" override_colour="var(--enabled-switch)" />
                } else {
                    <Svg svg_path="public/images/switch.svg" class="state-switch-disabled" override_colour="var(--disabled-switch)" />
                }
            </Button>
            <div>{&props.mod_data.name}</div>
            <div style="font-size: 0.9em;transform: translate(0, 0.1em)">{&props.mod_data.description}</div>
        </div>
    }
}