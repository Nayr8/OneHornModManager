use std::path::PathBuf;
use std::rc::Rc;
use yew::prelude::*;
use models::FileEntry;
use crate::bindings::FileBrowser;
use crate::components::{Spinner, Button};

#[derive(Properties, PartialEq)]
pub struct FileNavProps {
    pub current_path: UseStateHandle<Rc<PathBuf>>,
    pub current_entries: UseStateHandle<Vec<FileEntry>>,
}
#[function_component(FileNav)]
pub fn file_nav(props: &FileNavProps) -> Html {

    let common_paths = use_state(|| None);

    use_effect_with_deps(|common_paths| {
        FileBrowser::get_common_paths(common_paths.clone());
    }, common_paths.clone());

    html! {
        <div class="file-nav">
            if let Some(common_paths) = common_paths.as_ref() {
                <div style="text-align: center;padding-bottom: 0.3em;font-size: 1.2em">{"Quick Access"}</div>
                <div class="file-nav-common-paths">
                    {common_paths.iter().map(|(name, path)| {
                        let onclick = {
                            let path = path.clone();
                            let current_path = props.current_path.clone();
                            let current_entries = props.current_entries.clone();
                            move |_| {
                                FileBrowser::redirect_browser(path.clone());
                                FileBrowser::read_current_dir_into(current_path.clone(), current_entries.clone());
                            }
                        };
                        html! {
                            <Button onclick={onclick}>{name}</Button>
                        }
                    }).collect::<Html>()}
                </div>
            } else {
                <Spinner />
            }
        </div>
    }
}