use yew::platform::spawn_local;
use yew::prelude::*;
use crate::components::Button;

#[derive(Properties, PartialEq)]
pub struct BottomBarProps {
    pub console_open: UseStateHandle<bool>,
}

#[function_component(BottomBar)]
pub fn bottom_bar(props: &BottomBarProps) -> Html {
    let toggle_console = {
        let console_open = props.console_open.clone();
        move |_: MouseEvent| {
            console_open.set(!*console_open);
        }
    };

    let version = use_state(|| None);

    {
        let version = version.clone();
        use_effect(|| {
            spawn_local(async move {
                if let Ok(app_version) = tauri_sys::app::get_version().await {
                    version.set(Some(app_version));
                }
            });
        });
    }

    html! {
        <div class="bottom-bar">
            <Button onclick={toggle_console}>{"Toggle Console"}</Button>
            if let Some(version) = version.as_ref() {
                <div class="version-number">{format!("{}", version)}</div>
            }
        </div>
    }
}