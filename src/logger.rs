use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use yew::UseStateHandle;
use models::LogLine;
use crate::invoke;
use std::backtrace::Backtrace;


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
#[allow(unused_macros)]
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {crate::logger::Logger::log_critical(format_args!($($arg)*))};
}

#[derive(Serialize, Deserialize)]
struct LogArgs {
    message: String,
}

pub(crate) struct Logger;
#[allow(dead_code)]
impl Logger {
    pub fn init() {
        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            critical!("{info}\n{}", Backtrace::capture());
            default_panic(info)
        }));
    }

    pub fn read_logs_into(log_lines: UseStateHandle<Vec<LogLine>>) {
        spawn_local(async move {
            let messages = invoke("get_log_messages", JsValue::null()).await;
            let messages = serde_wasm_bindgen::from_value::<Vec<LogLine>>(messages).unwrap();
            log_lines.set(messages);
        });
    }

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
    pub fn log_critical(message: impl ToString) {
        let args = Self::build_args(message);
        spawn_local(async move {
            invoke("log_critical", args).await;
        });
    }

    fn build_args(message: impl ToString) -> JsValue {
        serde_wasm_bindgen::to_value(&LogArgs { message: message.to_string() }).unwrap()
    }
}
