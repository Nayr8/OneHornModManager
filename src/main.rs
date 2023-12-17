mod app;
mod menus;
mod helpers;
mod components;
mod models;
mod pages;
mod bindings;

use app::App;

#[derive(PartialEq)]
pub enum Status<T: PartialEq, ERR: PartialEq> {
    Loading,
    Loaded(T),
    Error(ERR),
}


fn main() {
    yew::Renderer::<App>::new().render();
}
