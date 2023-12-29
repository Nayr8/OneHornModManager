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
        Status::Loaded(mods_inner) => html! {
            <div class="mod-list">
                {mods_inner.iter().enumerate().map(|(index, mod_data)| html! {
                    <ModRow
                        mod_data={mod_data.clone()}
                        index={index}
                        mods={mods.clone()}
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
    mod_data: Mod,
    index: usize,
    mods: UseStateHandle<Status<Vec<Mod>, ()>>
}

#[function_component]
pub fn ModRow(props: &ModRowProps) -> Html {
    let index = props.index;
    let mods = props.mods.clone();
    let toggle_mod_enabled = move |_| {
        ManagerBindings::toggle_mod_enabled(index, mods.clone());
    };
    let mods = props.mods.clone();
    let delete_mod = move |_| {
        ManagerBindings::delete(index, mods.clone());
    };
    html! {
        <div class="mod">
            <Button onclick={delete_mod}>
                <Svg svg_path="public/images/delete.svg" width=1.4 height=1.4 />
            </Button>
            <Button onclick={toggle_mod_enabled} class="state-switch">
                if props.mod_data.enabled {
                    <Svg svg_path="public/images/switch.svg" width=2.0 height=2.0 override_colour="var(--enabled-switch)" />
                } else {
                    <Svg svg_path="public/images/switch.svg" width=2.0 height=2.0 flip_x=true override_colour="var(--disabled-switch)" />
                }
            </Button>
            <div>{&props.mod_data.name}</div>
            <div style="font-size: 0.9em;transform: translate(0, 0.1em)">{&props.mod_data.description}</div>
        </div>
    }
}