use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum LogSeverity {
    Info,
    Warn,
    Error,
    Critical,
}

impl Display for LogSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LogSeverity::Info => "Info",
            LogSeverity::Warn => "Warn",
            LogSeverity::Error => "Error",
            LogSeverity::Critical => "Critical",
        })
    }
}