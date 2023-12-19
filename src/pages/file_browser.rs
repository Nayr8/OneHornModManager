use yew::prelude::*;
use crate::bindings::{FileBrowserBindings, ManagerBindings};
use crate::components::{Button, ErrorMessage, Spinner, Svg};
use crate::helpers::localisation::LocalisationHelper;
use crate::models::{AppState, BrowserState, EntryType, FileEntry, ModDetails};
use crate::Status;

#[derive(Properties, PartialEq)]
pub struct FileBrowserProps {
    pub t: UseStateHandle<LocalisationHelper>,
    pub app_state: UseStateHandle<AppState>,
}

#[function_component]
pub fn FileBrowser(props: &FileBrowserProps) -> Html {
    let current_directory = use_state(|| None);
    let selected_file = use_state(|| None);
    let browser_state = use_state(|| BrowserState::FileBrowser);

    use_effect_with_deps({
        let current_directory = current_directory.clone();
        move |_| {
            FileBrowserBindings::read_current_dir(current_directory);
        }
    }, ());

    match *browser_state {
        BrowserState::FileBrowser => html! {
            <div class="file-browser">
                <Nav/>
                <Location t={props.t.clone()} current_dir={current_directory.clone()}/>
                <QuickAccess t={props.t.clone()} current_dir={current_directory.clone()} selected_file={selected_file.clone()}/>
                <FileList t={props.t.clone()} current_dir={current_directory.clone()} selected_file={selected_file.clone()}/>
                <SelectedFile t={props.t.clone()} selected_file={selected_file.clone()} browser_state={browser_state.clone()}/>
            </div>
        },
        BrowserState::AddMod => {
            html! {
                <AddMod
                    t={props.t.clone()}
                    selected_file={selected_file.clone()}
                    browser_state={browser_state.clone()}
                    app_state={props.app_state.clone()}
                />
            }
        }
    }


}

#[function_component]
fn Nav() -> Html {
    html! {
        <div class="nav">
            <Button>{"Back"}</Button>
            <Button>{"Forward"}</Button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LocationProps {
    t: UseStateHandle<LocalisationHelper>,
    current_dir: UseStateHandle<Option<(String, Vec<FileEntry>)>>
}

#[function_component]
fn Location(props: &LocationProps) -> Html {
    let html = match &*props.current_dir {
        Some((path, _)) => html! {
            <div style="text-align: left; width: 100%">{path}</div>
        },
        None => html! { <Spinner size=1.5/> }
    };
    html! {
        <div class="location">
            {html}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct QuickAccessProps {
    t: UseStateHandle<LocalisationHelper>,
    current_dir: UseStateHandle<Option<(String, Vec<FileEntry>)>>,
    selected_file: UseStateHandle<Option<FileEntry>>,
}

#[function_component]
fn QuickAccess(props: &QuickAccessProps) -> Html {
    let common_paths = use_state(|| None);

    use_effect_with_deps({
        let common_paths = common_paths.clone();
        move |_| {
            FileBrowserBindings::get_common_paths(common_paths)
        }
    }, ());

    let common_paths_html = match &*common_paths {
        Some(common_paths) => html! {
            <div class="quick-access-paths">
                {common_paths.iter().map(|(path_type, path)| {
                    let path = path.clone();
                    let current_dir = props.current_dir.clone();
                    let selected_file = props.selected_file.clone();
                    let onclick = move |_: MouseEvent| {
                        FileBrowserBindings::redirect_browser(path.clone(), current_dir.clone());
                        selected_file.set(None);
                    };
                    html! {
                        <Button class="common-path-button" onclick={onclick}>
                            <Svg svg_path={path_type.to_svg_path()} class="quick-access-image" />
                            <div style="font-size: 1.4em">{props.t.trans(path_type.to_translation_string())}</div>
                        </Button>
                    }
                }).collect::<Html>()}
            </div>
        },
        None => html! { <Spinner size=5.0/> }
    };

    html! {
        <div class="quick-access">
            <div style="font-size: 1.6em;text-align: center">{props.t.trans("page:file_browser:quick_access")}</div>
            {common_paths_html}
        </div>
    }
}


#[derive(Properties, PartialEq)]
struct FileListProps {
    t: UseStateHandle<LocalisationHelper>,
    current_dir: UseStateHandle<Option<(String, Vec<FileEntry>)>>,
    selected_file: UseStateHandle<Option<FileEntry>>,
}

#[function_component]
fn FileList(props: &FileListProps) -> Html {
    let html = match &*props.current_dir {
        Some((_, entries)) => {
            entries.iter().map(|entry| {
                let onclick = match entry.entry_type {
                    EntryType::File => Callback::from({
                        let entry = entry.clone();
                        let selected_file = props.selected_file.clone();
                        move |_| {
                            selected_file.set(Some(entry.clone()));
                        }
                    }),
                    EntryType::Directory => Callback::from({
                        let path = entry.path.clone();
                        let current_dir = props.current_dir.clone();
                        let selected_file = props.selected_file.clone();
                        move |_| {
                            FileBrowserBindings::redirect_browser(path.clone(), current_dir.clone());
                            selected_file.set(None);
                        }
                    })
                };
                html! {
                    <Button class="file" onclick={onclick}>
                        <Svg svg_path={entry.entry_type.to_svg_path()} style="display: flex;width: 1em;height: 1em;transform: translate(0, 0.2em)" />
                        <div>{&entry.file_name}</div>
                    </Button>
                }
            }).collect::<Html>()
        },
        None => html! {
            <div style="display: flex;align-items: center;height: 100%">
                <Spinner size=10.0/>
            </div>
        }
    };
    html! {
        <div class="files">
            {html}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SelectedFileProps {
    t: UseStateHandle<LocalisationHelper>,
    selected_file: UseStateHandle<Option<FileEntry>>,
    browser_state: UseStateHandle<BrowserState>,
}

#[function_component]
fn SelectedFile(props: &SelectedFileProps) -> Html {
    let html = match &*props.selected_file {
        Some(file) => {
            let browser_state = props.browser_state.clone();
            let onclick = move |_| {
                browser_state.set(BrowserState::AddMod)
            };
            html! {
            <>
                <div>{&*file.file_name}</div>
                <Button style="font-size: 1.5em;padding: 1em;transform: translate(0, -1em)" onclick={onclick}>
                    {props.t.trans("page:file_browser:choose_file")}
                </Button>
            </>
            }
        },
        None => html! {
        <>
            <div></div>
            <Button disabled=true style="color: var(--disabled);font-size: 1.5em;padding-right: 1em">
                {props.t.trans("page:file_browser:choose_file")}
            </Button>
        </>
        }
    };
    html! {
        <div class="selected">
            {html}
        </div>
    }
}



#[derive(Properties, PartialEq)]
struct AddModProps {
    t: UseStateHandle<LocalisationHelper>,
    selected_file: UseStateHandle<Option<FileEntry>>,
    browser_state: UseStateHandle<BrowserState>,
    app_state: UseStateHandle<AppState>,
}

#[function_component]
fn AddMod(props: &AddModProps) -> Html {
    let mod_details = use_state(|| Status::<ModDetails, ()>::Loading);

    use_effect_with_deps({
        let mod_details = mod_details.clone();
        let path = props.selected_file.as_ref().unwrap().path.clone();
        move |_| {
            ManagerBindings::get_mod_details(mod_details, path);
        }
    }, ());

    match &*mod_details {
        Status::Loading => {
            html! {
                <Spinner size=10.0 style="align-self: center;justify-self: center"/>
            }
        },
        Status::Loaded(details) => {
            let app_state = props.app_state.clone();
            let onclick = move |_| {
                ManagerBindings::add_current_mod();
                app_state.set(AppState::ModList);
            };
            html! {
                <div class="mod-details-outer">
                    <div class="mod-details">
                        <div style="font-size: 2em">{&details.name}</div>
                        <div style="font-size: 1.6em">{&details.description}</div>
                        <Button onclick={onclick} style="font-size: 1.6em">
                            {props.t.trans("page:file_browser:add_selected_mod")}
                        </Button>
                    </div>
                </div>
            }
        },
        Status::Error(()) => {
            html! {
                <ErrorMessage message={props.t.trans("page:file_browser:error_getting_mod_details")}/>
            }
        },
    }
}