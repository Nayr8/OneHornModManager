use yew::prelude::*;
use crate::bindings::ManagerBindings;
use crate::components::{Button, Svg, Tooltip};
use crate::helpers::localisation::LocalisationHelper;
use crate::models::AppState;

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
    pub t: UseStateHandle<LocalisationHelper>,
    pub state: UseStateHandle<AppState>,
}

#[function_component]
pub fn SideBar(props: &SideBarProps) -> Html {
    let disabled = match *props.state {
        AppState::ModList => 0,
        AppState::FileBrowser => 1,
    };

    let mut buttons = vec![
        SideBarButtonProps {
            tooltip: props.t.trans("menu:sidebar:home"),
            svg_path: "public/images/home.svg".into(),
            onclick: Callback::from({
                let state = props.state.clone();
                move |_| { state.set(AppState::ModList); }
            }),
            disabled: false,
        },
        SideBarButtonProps {
            tooltip: props.t.trans("menu:sidebar:add_mod"),
            svg_path: "public/images/add_mod.svg".into(),
            onclick: Callback::from({
                let state = props.state.clone();
                move |_| { state.set(AppState::FileBrowser); }
            }),
            disabled: false,
        },
        SideBarButtonProps {
            tooltip: props.t.trans("menu:sidebar:change_profile"),
            svg_path: "public/images/change_profile.svg".into(),
            onclick: Default::default(),
            disabled: true,
        },
        SideBarButtonProps {
            tooltip: props.t.trans("menu:sidebar:settings"),
            svg_path: "public/images/settings.svg".into(),
            onclick: Default::default(),
            disabled: true,
        },
        SideBarButtonProps {
            tooltip: props.t.trans("menu:sidebar:apply"),
            svg_path: "public/images/apply.svg".into(),
            onclick: Callback::from(|_| {
                ManagerBindings::apply();
            }),
            disabled: false,
        },
    ];

    buttons[disabled].disabled = true;

    html! {
        <div class="side-bar">
            { for buttons.into_iter().map(|props| html! {
                <SideBarButton
                    tooltip={props.tooltip}
                    svg_path={props.svg_path}
                    onclick={props.onclick}
                    disabled={props.disabled}
                />
            }) }
            <div/>
            <SideBarButton
                tooltip={props.t.trans("menu:sidebar:logs_console")}
                svg_path="public/images/logs_console.svg"
                onclick={|_| {}}
                disabled=true
            />
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct SideBarButtonProps {
    tooltip: String,
    svg_path: String,
    onclick: Callback<MouseEvent>,
    disabled: bool,
}

#[function_component(SideBarButton)]
fn side_bar_button(props: &SideBarButtonProps) -> Html {
    let override_colour = if props.disabled {
        "var(--disabled)"
    } else {
        "var(--text)"
    };
    html! {
        <Tooltip tooltip={props.tooltip.clone()} disabled={props.disabled}>
            <Button onclick={props.onclick.clone()} disabled={props.disabled}>
                <Svg svg_path={props.svg_path.clone()} class="menu-image" override_colour={override_colour}/>
            </Button>
        </Tooltip>
    }
}