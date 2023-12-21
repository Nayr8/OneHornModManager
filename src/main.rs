mod app;
mod menus;
mod helpers;
mod components;
mod models;
mod pages;
mod bindings;

use log::LevelFilter;
use app::App;
use crate::bindings::UILogger;

#[derive(PartialEq)]
pub enum Status<T: PartialEq, ERR: PartialEq> {
    Loading,
    Loaded(T),
    Error(ERR),
}

fn main() {
    log::set_boxed_logger(Box::new(UILogger)).unwrap();
    log::set_max_level(LevelFilter::Info);
    bindings::bind_logging();
    yew::Renderer::<App>::new().render();
}
