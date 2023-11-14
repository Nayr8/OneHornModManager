mod app;
mod components;
mod logger;
mod bindings;
mod pages;
pub mod bottom_bar;
pub mod console;
pub mod top_bar;
mod helpers;

use wasm_bindgen::prelude::*;
use app::App;
use crate::logger::Logger;


fn main() {
    Logger::init();
    yew::Renderer::<App>::new().render();
}
