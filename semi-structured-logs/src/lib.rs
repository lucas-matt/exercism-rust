// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use crate::LogLevel::{Info, Warning, Error};

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let level = match level {
        Info => "INFO",
        Warning => "WARNING",
        Error => "ERROR"
    };
    format!("[{}]: {}", level, message)
}
pub fn info(message: &str) -> String {
    log(Info, message)
}
pub fn warn(message: &str) -> String {
    log(Warning, message)
}
pub fn error(message: &str) -> String {
    log(Error, message)
}
