use chrono::Local;
use std::fmt;
use tauri_plugin_log::logger::Logger;
use tauri::{Manager, Window};

use tauri::api::log::{ConsoleLogLevel, Logger};

pub fn log(window: &tauri::Window, level: ConsoleLogLevel, message: &str) {
    // Access the Logger object (if it's available) to log to multiple targets
    if let Some(logger) = window.app_handle().state::<Logger>() {
        logger.log(window, level, message);
    } else {
        // Fallback to basic logging
        tauri::Window::emit_log(window, level, message);
    }
}
