use yew::prelude::*;
use crate::helpers::localisation::LocalisationHelper;


#[derive(Properties, PartialEq)]
pub struct SettingsProps {
    pub t: UseStateHandle<LocalisationHelper>,
}

#[function_component]
pub fn Settings(props: &SettingsProps) -> Html {
    html! {

    }
}