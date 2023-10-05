use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileNavProps {

}
#[function_component(FileNav)]
pub fn file_nav(_props: &FileNavProps) -> Html {
    html! {
        <div class="file-nav" />
    }
}