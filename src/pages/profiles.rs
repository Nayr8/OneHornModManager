use yew::prelude::*;
use crate::bindings::ManagerBindings;
use crate::helpers::localisation::LocalisationHelper;


#[derive(Properties, PartialEq)]
pub struct ProfilesProps {
    pub t: UseStateHandle<LocalisationHelper>,
}

#[function_component]
pub fn Profiles(props: &ProfilesProps) -> Html {
    let profile = use_state(|| None);
    let profiles = use_state(|| None);
    use_effect_with_deps({
        let profile = profile.clone();
        let profiles = profiles.clone();
        move |_| {
            ManagerBindings::get_current_profile(profile);
            ManagerBindings::get_profiles(profiles);
        }
    }, ());

    if profile.is_none() || profiles.is_none() {
        return html! {};
    }

    let current_profile = profile.as_ref().unwrap();
    let profiles = profiles.as_ref().unwrap();
    html! {
        <>
            <div>{&format!("{}: {}", current_profile.1, current_profile.0)}</div>
        </>
    }
}