use yew::prelude::*;
use crate::bindings::FileBrowserBindings;
use crate::components::{Button, Spinner, Svg};
use crate::helpers::localisation::LocalisationHelper;

#[derive(Properties, PartialEq)]
pub struct FileBrowserProps {
    pub t: UseStateHandle<LocalisationHelper>,
}

#[function_component]
pub fn FileBrowser(props: &FileBrowserProps) -> Html {
    html! {
        <div class="file-browser">
            <Nav t={props.t.clone()}/>
            <div class="location">{"Location"}</div>
            <QuickAccess t={props.t.clone()}/>
            <div class="files">{"files"}</div>
            <div class="selected">{"Selected"}</div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct NavProps {
    pub t: UseStateHandle<LocalisationHelper>,
}

#[function_component]
fn Nav(props: &NavProps) -> Html {
    html! {
        <div class="nav">
            <Button>{"Back"}</Button>
            <Button>{"Forward"}</Button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct QuickAccessProps {
    pub t: UseStateHandle<LocalisationHelper>,
}

#[function_component]
fn QuickAccess(props: &QuickAccessProps) -> Html {
    let common_paths = use_state(|| None);

    use_effect({
        let common_paths = common_paths.clone();
        move || {
            FileBrowserBindings::get_common_paths(common_paths)
        }
    });

    let common_paths_html = match &*common_paths {
        Some(common_paths) => html! {
            <div class="quick-access-paths">
                {common_paths.iter().map(|(path_type, path)| {
                    html! {
                        <Button class="common-path-button">
                            <Svg svg_path={path_type.to_svg_path()} class="quick-access-image" />
                            <div style="font-size: 1.2em">{props.t.trans(path_type.to_translation_string())}</div>
                        </Button>
                    }
                }).collect::<Html>()}
            </div>
        },
        None => html! { <Spinner size=10.0/> }
    };

    html! {
        <div class="quick-access">
            <div style="font-size: 1.4em;text-align: center">{props.t.trans("page:file_browser:quick_access")}</div>
            {common_paths_html}
        </div>
    }
}