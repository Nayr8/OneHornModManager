use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::UseStateHandle;
use models::LogLine;
use std::backtrace::Backtrace;
use tauri_sys::tauri;
use crate::bindings::Null;


#[allow(unused_macros)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {$crate::logger::Logger::log_info(format_args!($($arg)*).to_string())};
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {$crate::logger::Logger::log_warn(format_args!($($arg)*).to_string())};
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {$crate::logger::Logger::log_error(format_args!($($arg)*).to_string())};
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {$crate::logger::Logger::log_critical(format_args!($($arg)*).to_string())};
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
            default_panic(info);
        }));
    }

    pub fn read_logs_into(log_lines: UseStateHandle<Vec<LogLine>>) {
        spawn_local(async move {
            log_lines.set(tauri::invoke("get_log_messages", &Null).await.unwrap());
        });
    }

    pub fn log_info(message: String) {
        spawn_local(async move {
            let _: () = tauri::invoke("log_info", &LogArgs { message }).await.unwrap();
        });
    }
    pub fn log_warn(message: String) {
        spawn_local(async move {
            let _: () = tauri::invoke("log_warn", &LogArgs { message }).await.unwrap();
        });
    }
    pub fn log_error(message: String) {
        spawn_local(async move {
            let _: () = tauri::invoke("log_error", &LogArgs { message }).await.unwrap();
        });
    }
    pub fn log_critical(message: String) {
        spawn_local(async move {
            let _: () = tauri::invoke("log_critical", &LogArgs { message }).await.unwrap();
        });
    }
}
