use js_sys::{Function, Object};
use log::{Level, Log, Metadata, Record};
use serde::{Deserialize, Serialize};
use tauri_sys::tauri;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;
use yew::platform::spawn_local;

pub fn bind_logging() {
    fn single_bind(name: &'static str, command: &'static str) {
        let window = web_sys::window().unwrap();
        let console = js_sys::Reflect::get(&window, &"console".into()).unwrap();

        let old_log = js_sys::Reflect::get(&console, &name.into()).unwrap().dyn_into::<Function>().unwrap();

        let function = Closure::wrap(Box::new(move |message: JsValue| {
            log_inner(message, name, command, &old_log);
        }) as Box<dyn FnMut(JsValue)>);

        js_sys::Reflect::set(&console, &name.into(), function.as_ref().unchecked_ref()).unwrap();
        function.forget();
    }

    single_bind("log", "info");
    single_bind("warn", "warn");
    single_bind("error", "error");
}

#[derive(Serialize, Deserialize)]
pub struct LogArgs {
    message: String,
    target: String,
    module_path: Option<String>,
    file: Option<String>,
    line: Option<u32>,
}

fn log_inner(message: JsValue, name: &'static str, command: &'static str, old_log: &Function) {
    let old_log_message = message.clone();
    spawn_local(async move {
        let args = serde_wasm_bindgen::from_value::<LogArgs>(message.clone()).unwrap_or_else(|_| {
            let object_message: Object = message.clone().into();
            let string_message = object_message.to_string();
            let message = string_message.as_string().unwrap_or_default();
            LogArgs {
                message,
                target: format!("console.{name}"),
                module_path: None,
                file: None,
                line: None,
            }
        });

        let _: () = tauri::invoke(command, &args).await.unwrap();
    });
    old_log.call1(&JsValue::null(), &old_log_message).unwrap();
}

pub struct UILogger;

impl Log for UILogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let log_args = LogArgs {
            message: format!("{}", record.args()),
            target: record.target().to_owned(),
            module_path: record.module_path().map(|s| s.to_owned()),
            file: record.file().map(|s| s.to_owned()),
            line: record.line(),
        };

        let args = serde_wasm_bindgen::to_value(&log_args).unwrap();
        match record.level() {
            Level::Error => console::error_1(&args),
            Level::Warn => console::warn_1(&args),
            Level::Info => console::log_1(&args),
            Level::Debug | Level::Trace => unimplemented!()
        }
    }

    fn flush(&self) {}
}