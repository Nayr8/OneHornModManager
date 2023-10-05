use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use crate::invoke;


#[allow(unused_macros)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {crate::logger::Logger::log_info(format_args!($($arg)*))};
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {crate::logger::Logger::log_warn(format_args!($($arg)*))};
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {crate::logger::Logger::log_error(format_args!($($arg)*))};
}

#[derive(Serialize, Deserialize)]
struct LogArgs {
    message: String,
}

pub(crate) struct Logger;
#[allow(dead_code)]
impl Logger {
    pub fn log_info(message: impl ToString) {
        let args = Self::build_args(message);
        spawn_local(async move {
            invoke("log_info", args).await;
        });
    }
    pub fn log_warn(message: impl ToString) {
        let args = Self::build_args(message);
        spawn_local(async move {
            invoke("log_warn", args).await;
        });
    }
    pub fn log_error(message: impl ToString) {
        let args = Self::build_args(message);
        spawn_local(async move {
            invoke("log_error", args).await;
        });
    }

    fn build_args(message: impl ToString) -> JsValue {
        serde_wasm_bindgen::to_value(&LogArgs { message: message.to_string() }).unwrap()
    }
}
