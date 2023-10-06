mod app;
mod components;
mod logger;
mod bindings;

use wasm_bindgen::prelude::*;
use app::App;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn main() {
    info!("Info Test");
    warn!("Warning Test");
    error!("Error Test");
    yew::Renderer::<App>::new().render();
}
