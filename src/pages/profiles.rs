use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::bindings::ManagerBindings;
use crate::components::{Button, Spinner, Svg};
use crate::helpers::localisation::LocalisationHelper;


#[derive(Properties, PartialEq)]
pub struct ProfilesProps {
    pub t: UseStateHandle<LocalisationHelper>,
}

#[function_component]
pub fn Profiles(props: &ProfilesProps) -> Html {
    let profile = use_state(|| None);
    let profiles = use_state(|| None);
    let input = use_state(|| String::new());

    use_effect_with_deps({
        let profile = profile.clone();
        let profiles = profiles.clone();
        move |_| {
            ManagerBindings::get_current_profile(profile);
            ManagerBindings::get_profiles(profiles);
        }
    }, ());

    if profile.is_none() || profiles.is_none() {
        return html! { <Spinner size=10.0 /> };
    }

    let add_profile = {
        let profile = profile.clone();
        let profiles = profiles.clone();
        let input = input.clone();
        move |_| {
            let name = input.as_str();
            input.set(String::new());
            ManagerBindings::create_profile(name.to_owned(), profile.clone(), profiles.clone());
        }
    };

    let input_handler = {
        let input = input.clone();
        move |event: InputEvent| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let value = target.value();
            input.set(value);
        }
    };

    let current_profile = profile.as_ref().unwrap();
    let profiles_unwrapped = profiles.as_ref().unwrap();
    html! {
        <div class="profiles">
            <div class="current-profile">
                <div style="align-self: end;font-size: 2em;padding-bottom: 0.5em">{props.t.trans("page:profiles:current_profile")}</div>
                <div style="align-self: start;font-size: 1.6em">{&current_profile.1}</div>
            </div>
            <div class="profiles-list-outer">
                <div style="font-size: 1.4em;text-align: center;margin-bottom: 1em">
                    {props.t.trans("page:profiles:profiles")}
                </div>
                <div/>
                <div class="profiles-list">
                    <input type="text" value={input.as_str().to_owned()} class="create-profile-input" oninput={input_handler}/>
                    <Button onclick={add_profile} class="create-profile-button">{props.t.trans("page:profiles:create_account")}</Button>
                    {profiles_unwrapped.iter().map(|(id, name)| {
                        let profile = profile.clone();
                        let profiles = profiles.clone();
                        let profiles2 = profiles.clone();
                        let id = id.clone();
                        let onclick = move |_| {
                            ManagerBindings::switch_profile(id, profile.clone(), profiles.clone());
                        };
                        let delete = move |_| {
                            ManagerBindings::delete_profile(id, profiles2.clone());
                        };
                        html! {
                            <>
                                <Button class="profile" onclick={onclick}>{name}</Button>
                                <Button style="align-self: center;justify-self: center" onclick={delete}>
                                    <Svg svg_path="public/images/delete.svg" style="width: 1.2em;height: 1.2em"/>
                                </Button>
                            </>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
    }
}