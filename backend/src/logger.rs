use std::backtrace::Backtrace;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use spin::Mutex;
use models::{LogLine, LogSeverity};

static LOGGER: Mutex<Logger> = Mutex::new(Logger::new());

#[allow(unused_macros)]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {crate::logger::Logger::log_trace(format_args!($($arg)*))};
}
#[allow(unused_macros)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {crate::logger::Logger::log_debug(format_args!($($arg)*))};
}
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

#[tauri::command(rename_all = "snake_case")]
pub fn log_trace(message: String) {
    Logger::log(LogSeverity::Trace, message)
}

#[tauri::command(rename_all = "snake_case")]
pub fn log_debug(message: String) {
    Logger::log_debug(message)
}

#[tauri::command(rename_all = "snake_case")]
pub fn log_info(message: String) {
    Logger::log_info(message)
}

#[tauri::command(rename_all = "snake_case")]
pub fn log_warn(message: String) {
    Logger::log_warn(message)
}

#[tauri::command(rename_all = "snake_case")]
pub fn log_error(message: String) {
    Logger::log_error(message)
}

#[tauri::command(rename_all = "snake_case")]
pub fn log_critical(message: String) {
    Logger::log_critical(message)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_log_messages() -> Vec<LogLine> {
    LOGGER.lock().log_lines.clone()
}

pub(crate) struct Logger {
    log_lines: Vec<LogLine>,
}

#[allow(dead_code)]
impl Logger {
    const fn new() -> Logger {
        Logger {
            log_lines: Vec::new(),
        }
    }

    pub fn init() {
        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            critical!("{info}\n{}", Backtrace::capture());
            default_panic(info)
        }));
    }

    // Must NOT panic as it is also called in the panic handler
    pub fn log(severity: LogSeverity, message: impl ToString) {
        let timestamp = Local::now();
        let log_line = LogLine {
            severity,
            timestamp: timestamp.timestamp(),
            message: message.to_string(),
        };

        println!("{log_line}");

        let mut log_file = match OpenOptions::new().create(true).append(true).open("../target/mod_manager.log") {
            Ok(log_file) => log_file,
            Err(e) => {
                println!("Could not write log to file: {e}");
                LOGGER.lock().log_lines.push(log_line);
                return;
            }
        };

        match log_file.write(log_line.to_string().as_bytes()) {
            Ok(_) => match log_file.write(b"\n") {
                Ok(_) => if let Err(e) = log_file.flush() {
                    println!("Could not write log to file: {e}");
                },
                Err(e) => println!("Could not write log to file: {e}"),
            },
            Err(e) => println!("Could not write log to file: {e}"),
        }
        LOGGER.lock().log_lines.push(log_line)
    }

    pub fn log_trace(message: impl ToString) {
        Logger::log(LogSeverity::Trace, message);
    }

    pub fn log_debug(message: impl ToString) {
        Logger::log(LogSeverity::Debug, message);
    }

    pub fn log_info(message: impl ToString) {
        Logger::log(LogSeverity::Info, message);
    }

    pub fn log_warn(message: impl ToString) {
        Logger::log(LogSeverity::Warn, message);
    }

    pub fn log_error(message: impl ToString) {
        Logger::log(LogSeverity::Error, message);
    }

    pub fn log_critical(message: impl ToString) {
        Logger::log(LogSeverity::Critical, message);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new()
    }
}