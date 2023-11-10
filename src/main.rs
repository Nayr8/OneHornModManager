mod app;
mod components;
mod logger;
mod bindings;
mod pages;
pub mod bottom_bar;
pub mod console;
pub mod top_bar;

use wasm_bindgen::prelude::*;
use app::App;
use crate::logger::Logger;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn main() {
    Logger::init();
    yew::Renderer::<App>::new().render();
}
