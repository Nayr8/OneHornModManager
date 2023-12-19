use yew::prelude::*;
use crate::bindings;
use crate::helpers::localisation::LocalisationHelper;
use crate::menus::side_bar::SideBar;
use crate::models::AppState;
use crate::pages::{FileBrowser, ModList, Profiles, Settings};


#[function_component]
pub fn App() -> Html {
    let t = use_state(|| LocalisationHelper::default());
    let state = use_state(|| AppState::ModList);

    use_effect_with_deps({
        bindings::bind_logging();

        let t = t.clone();
        move |()| {
            LocalisationHelper::change("en-GB".into(), t);
        }
    }, ());

    html! {
        <div class="app">
            <SideBar t={t.clone()} state={state.clone()} />
            {match *state {
                AppState::ModList => html! { <ModList t={t.clone()}/> },
                AppState::FileBrowser => html! { <FileBrowser t={t.clone()} app_state={state.clone()}/> },
                AppState::Profiles => html! { <Profiles t={t.clone()}/> },
                AppState::Settings => html! { <Settings t={t.clone()}/> },
            }}
        </div>
    }
}
