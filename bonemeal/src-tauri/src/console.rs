use chrono::Local;
use std::fmt;
use tauri::{AppHandle, Manager};
use tauri::Window;

#[derive(Debug)]
pub enum ConsoleLogLevel {
    Info,
    Warning,
    Error,
}

impl fmt::Display for ConsoleLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsoleLogLevel::Info => write!(f, "[INFO]"),
            ConsoleLogLevel::Warning => write!(f, "[WARNING]"),
            ConsoleLogLevel::Error => write!(f, "[ERROR]"),
        }
    }
}

pub fn log(window: &Window, level: ConsoleLogLevel, message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("{} {} {}", timestamp, level, message);

    window.eval(&format!("appendToConsoleLog('{:?}', '{}')", level, message)).unwrap();
}