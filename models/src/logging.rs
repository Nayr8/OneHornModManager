use std::fmt::{Display, Formatter};
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum LogSeverity {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl Display for LogSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LogSeverity::Trace => "Trace",
            LogSeverity::Debug => "Debug",
            LogSeverity::Info => "Info",
            LogSeverity::Warn => "Warn",
            LogSeverity::Error => "Error",
            LogSeverity::Critical => "Critical",
        })
    }
}



#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct LogLine {
    pub severity: LogSeverity,
    pub timestamp: i64,
    pub message: String,
}

impl Display for LogLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let timestamp = match DateTime::from_timestamp(self.timestamp, 0) {
            Some(timestamp) => timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => String::from("00-00-00 00:00:00"),
        };
        let severity = &self.severity;
        let message = &self.message;
        write!(f, "[{timestamp}] [{severity}]: {message}")
    }
}