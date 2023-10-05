use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileExplorerLocationProps {
    pub current_directory: String,
}
#[function_component(FileExplorerLocation)]
pub fn file_explorer_location(props: &FileExplorerLocationProps) -> Html {


    html! {
        <div class="file-location-outer">
            <div class="file-location element">{&props.current_directory}</div>
        </div>
    }
}