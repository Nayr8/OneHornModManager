use std::rc::Rc;
use yew::prelude::*;
use models::{Mod, Status};
use models::Status::Loaded;
use crate::bindings::ModManager;
use crate::components::Spinner;
use crate::components::spinner::SpinnerSize;

#[derive(Properties, PartialEq)]
pub struct ProfilesProps {
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub mods: UseStateHandle<Status<Rc<Vec<Mod>>>>,
}

#[function_component(Profiles)]
pub fn profiles(props: &ProfilesProps) -> Html {
    let profiles = use_state(|| Status::Loading);
    let open = use_state(|| false);

    use_effect_with_deps(|profiles| {
        ModManager::get_profiles(profiles.clone());
    }, profiles.clone());

    let Loaded(profiles_ref) = profiles.as_ref() else {
        return html! {
            <div class="element">
                <Spinner size={SpinnerSize::Small} />
            </div>
        };
    };

    let toggle_open = {
        let open = open.clone();
        move |_: MouseEvent| {
            open.set(!*open);
        }
    };

    let profiles_list_html = profiles_ref.profiles.iter().map(|(index, profile)| {
        let onclick = {
            let open = open.clone();
            let profiles = profiles.clone();
            let selected_mod = props.selected_mod.clone();
            let mods = props.mods.clone();
            let index = *index;
            move |_: MouseEvent| {
                open.set(false);
                mods.set(Status::Loading);
                ModManager::switch_profile(index);
                ModManager::get_profiles(profiles.clone());
                selected_mod.set(None);
                ModManager::get_mods(mods.clone());
            }
        };
        html! {
            <div class="profile make-element-button" onclick={onclick}>{profile}</div>}
    }).collect::<Html>();

    html! {
        <div class="element profiles">
            <div class="selected-profile make-element-button" onclick={toggle_open}>
                {format!("Profile: {}", profiles_ref.profiles[&profiles_ref.current_profile])}
            </div>
            if *open {
                <div class="profiles-list">
                    {profiles_list_html}
                    <div class="profile make-element-button new-profile">{"+"}</div>
                </div>
            }
        </div>
    }
}