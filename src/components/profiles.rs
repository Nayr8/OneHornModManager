use yew::prelude::*;
use crate::bindings::ModManager;
use crate::components::{Spinner, Button};
use crate::components::spinner::SpinnerSize;

#[derive(Properties, PartialEq)]
pub struct ProfilesProps {

}

#[function_component(Profiles)]
pub fn profiles(props: &ProfilesProps) -> Html {
    let profiles = use_state(|| None);

    use_effect_with_deps(|profiles| {
        ModManager::get_profiles(profiles.clone());
    }, profiles.clone());

    let Some(profiles) = profiles.as_ref() else {
        return html! {
            <div class="element">
                <Spinner size={SpinnerSize::Small} />
            </div>
        };
    };

    html! {
        <div>
            <Button>{format!("Profile: {}", profiles.profiles[&profiles.current_profile])}</Button>
        </div>
    }
}