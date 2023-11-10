use yew::prelude::*;
use crate::components::Element;

#[derive(Properties, PartialEq)]
pub struct FileExplorerLocationProps {
    pub current_directory: String,
}
#[function_component(FileExplorerLocation)]
pub fn file_explorer_location(props: &FileExplorerLocationProps) -> Html {
    html! {
        <div class="file-location-outer">
            <Element class="file-location">{&props.current_directory}</Element>
        </div>
    }
}