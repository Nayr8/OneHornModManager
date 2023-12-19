use js_sys::Object;
use serde::{Serialize, Serializer};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::platform::spawn_local;

mod file_browser;
mod manager;

pub use file_browser::FileBrowserBindings;
pub use manager::ManagerBindings;

pub struct Null;

impl Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_none()
    }
}

pub fn bind_logging() {
    fn single_bind(name: &'static str, command: &'static str) {
        let window = web_sys::window().unwrap();
        let console = js_sys::Reflect::get(&window, &"console".into()).unwrap();

        let old_log = js_sys::Reflect::get(&console, &name.into()).unwrap().dyn_into::<js_sys::Function>().unwrap();

        let function = Closure::wrap(Box::new(move |message: JsValue| {
            let object_message: Object = message.clone().into();
            let string_message = object_message.to_string();
            spawn_local(async move {
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
                    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
                }
                let obj = Object::new();
                let key = JsValue::from_str("message");

                js_sys::Reflect::set(&obj, &key, &JsValue::from(string_message)).expect("Failed to set property");

                invoke(command, JsValue::from(obj)).await;
            });
            old_log.call1(&JsValue::null(), &message).unwrap();
        }) as Box<dyn FnMut(JsValue)>);

        js_sys::Reflect::set(&console, &name.into(), function.as_ref().unchecked_ref()).unwrap();
        function.forget();
    }

    single_bind("log", "info");
    single_bind("warn", "warn");
    single_bind("error", "error");
}