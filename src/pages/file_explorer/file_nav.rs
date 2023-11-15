use std::path::PathBuf;
use std::sync::Arc;
use yew::prelude::*;
use models::{FileEntry, Status};
use crate::bindings::FileBrowser;
use crate::components::{Spinner, Button};

#[derive(Properties, PartialEq)]
pub struct FileNavProps {
    pub current_path: UseStateHandle<Arc<PathBuf>>,
    pub current_entries: UseStateHandle<Vec<FileEntry>>,
    pub navigation_enabled_state: UseStateHandle<(bool, bool)>,
}
#[function_component(FileNav)]
pub fn file_nav(props: &FileNavProps) -> Html {

    let common_paths = use_state(|| Status::Loading);

    use_effect_with_deps(|common_paths| {
        FileBrowser::get_common_paths(common_paths.clone());
    }, common_paths.clone());

    html! {
        <div class="file-nav">
            if let Status::Loaded(common_paths) = common_paths.as_ref() {
                <div style="text-align: center;padding-bottom: 0.3em;font-size: 1.2em">{"Quick Access"}</div>
                <div class="file-nav-common-paths">
                    {common_paths.iter().map(|(name, path)| {
                        let onclick = {
                            let path = path.clone();
                            let current_path = props.current_path.clone();
                            let current_entries = props.current_entries.clone();
                            let navigation_enabled_state = props.navigation_enabled_state.clone();
                            move |_| {
                                FileBrowser::redirect_browser(path.clone());
                                FileBrowser::read_current_dir_into(current_path.clone(), current_entries.clone());
                                FileBrowser::get_navigation_enabled_state(navigation_enabled_state.clone());
                            }
                        };
                        let img_src = match name.as_str() {
                            "Home" => Some("public/file_browser_home.png"),
                            "Documents" => Some("public/file_browser_documents.png"),
                            "Downloads" => Some("public/file_browser_downloads.png"),
                            "Desktop" => Some("public/file_browser_desktop.png"),
                            _ => None
                        };

                        html! {
                            <Button onclick={onclick}>
                                if let Some(img_src) = img_src {
                                    <img src={img_src} style="margin-right: 0.35em" />
                                }
                                {name}
                            </Button>
                        }
                    }).collect::<Html>()}
                </div>
            } else {
                <Spinner />
            }
        </div>
    }
}