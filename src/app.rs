use yew::prelude::*;
use crate::helpers::localisation::LocalisationHelper;
use crate::menus::side_bar::SideBar;
use crate::models::{Mod, AppState};
use crate::pages::{FileBrowser, ModList};
use crate::Status;


#[function_component]
pub fn App() -> Html {
    let t = use_state(|| LocalisationHelper::default());
    let state = use_state(|| AppState::FileBrowser);
    let mods = use_state(|| Status::Loaded::<_, ()>(vec![
        Mod {
            name: "BetterHotBar2_UW_4x31".into(),
            description: "BetterHotBar2_UW_4x31".into(),
            enabled: true,
        }
    ]));

    use_effect_with_deps({
        let t = t.clone();
        move |()| {
            LocalisationHelper::change("en-GB".into(), t);
        }
    }, ());

    html! {
        <div class="app">
            <SideBar t={t.clone()} state={state.clone()} />
            {match *state {
                AppState::ModList => html! { <ModList t={t.clone()} mods={mods.clone()}/> },
                AppState::FileBrowser => html! { <FileBrowser t={t.clone()}/> }
            }}

        </div>
    }
}
