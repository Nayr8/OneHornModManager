use std::rc::Rc;
use yew::prelude::*;
use models::{Mod, Status};
use models::Status::Loaded;
use crate::bindings::ModManager;
use crate::components::{Spinner, Button};
use crate::components::spinner::SpinnerSize;
use crate::helpers::DomHelper;

#[derive(Properties, PartialEq)]
pub struct ProfilesProps {
    pub selected_mod: UseStateHandle<Option<usize>>,
    pub mods: UseStateHandle<Status<Rc<Vec<Mod>>>>,
    #[prop_or_default]
    pub disabled: bool,
    pub profile_open: UseStateHandle<bool>,
    pub profile_create_new: UseStateHandle<bool>,
}

#[function_component(Profiles)]
pub fn profiles(props: &ProfilesProps) -> Html {
    let profiles = use_state(|| Status::Loading);

    use_effect_with_deps(|profiles| {
        ModManager::get_profiles(profiles.clone());
    }, profiles.clone());

    match profiles.as_ref() {
        Status::Loading => html! {
            <div class="element">
                <Spinner size={SpinnerSize::Small} />
            </div>
        },
        Loaded(profiles_ref) => {
            let toggle_open = {
                let profile_create_new = props.profile_create_new.clone();
                let profile_open = props.profile_open.clone();
                move |_: MouseEvent| {
                    profile_open.set(!*profile_open);
                    profile_create_new.set(false);
                }
            };

            let profiles_list_html = profiles_ref.profiles.iter().map(|(index, profile)| {
                if *index == profiles_ref.current_profile { return html! {<div></div>} }
                let onclick = {
                    let profile_open = props.profile_open.clone();
                    let profile_create_new = props.profile_create_new.clone();
                    let profiles = profiles.clone();
                    let selected_mod = props.selected_mod.clone();
                    let mods = props.mods.clone();
                    let index = *index;
                    move |_: MouseEvent| {
                        profile_open.set(false);
                        profile_create_new.set(false);
                        mods.set(Status::Loading);
                        ModManager::switch_profile(index);
                        ModManager::get_profiles(profiles.clone());
                        selected_mod.set(None);
                        ModManager::get_mods(mods.clone());
                    }
                };
                html! { <div class="profile make-element-button" onclick={onclick}>{profile}</div>}
            }).collect::<Html>();

            let open_create_profile = {
                let profile_create_new = props.profile_create_new.clone();
                move |_: MouseEvent| {
                    profile_create_new.set(true);
                }
            };

            let create_profile = {
                let profile_create_new = props.profile_create_new.clone();
                let profile_open = props.profile_open.clone();
                let profiles = profiles.clone();
                let selected_mod = props.selected_mod.clone();
                let mods = props.mods.clone();
                move |_: MouseEvent| {
                    let Some(profile) = DomHelper::read_input("new-profile-input") else { return };
                    if !profile.is_empty() {
                        ModManager::create_profile(profile);
                        profile_create_new.set(false);
                        profile_open.set(false);
                        ModManager::get_profiles(profiles.clone());
                        selected_mod.set(None);
                        ModManager::get_mods(mods.clone());
                    }
                }
            };


            html! {
                <div class="element profiles">
                    if props.disabled {
                        <div class="selected-profile make-element-disabled">
                            {format!("Profile: {}", profiles_ref.profiles[&profiles_ref.current_profile])}
                        </div>
                    } else {
                        <div class={classes!("selected-profile", "make-element-button", if props.disabled { Some("make-element-disabled") } else { None })} onclick={toggle_open}>
                            {format!("Profile: {}", profiles_ref.profiles[&profiles_ref.current_profile])}
                        </div>
                    }

                    if *props.profile_open {
                        <div class="profiles-list">
                            {profiles_list_html}
                            if *props.profile_create_new {
                                <div class="new-profile">
                                    <input id="new-profile-input" type="text" class="profile new-profile-input" />
                                    <Button onclick={create_profile}>{"Submit"}</Button>
                                </div>
                            } else {
                                <div class="profile make-element-button new-profile-text" onclick={open_create_profile}>{"+"}</div>
                            }
                        </div>
                    }
                </div>
            }
        },
        Status::Error(_) => unimplemented!()
    }
}