// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod console;

#[macro_use]
extern crate lazy_static;
extern crate tera;

use std::collections::HashMap;
use tauri::Manager;
use tera::{Context, Tera};
use window_shadows::set_shadow;

fn page(value: &serde_json::Value, _args: &HashMap<String, serde_json::Value>) -> tera::Result<serde_json::Value> {
    if let serde_json::Value::String(s) = value {
        // The value is a string, so we assume it's a URL.
        return Ok(serde_json::Value::String(format!("window.location.href = '{}';", s)));
    }

    // The value is not a string, so we return it unchanged.
    Ok(value.clone())
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("../src/ui/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera.register_filter("page", page);
        tera
    };
}


fn main() {
    // Define routes and their corresponding template files
    let routes = vec![
        ("/", "index.html"),
        ("/tools", "tools.html"),
        ("/stats", "stats.html"),
        ("/log", "log.html"),
        ("/settings", "settings.html"),
    ];
    tauri::Builder::default()
        .setup(move |app| {
            #[cfg(any(windows, target_os = "macos"))]
            {
                let window = app.get_window("main").unwrap();
                set_shadow(&window, true).unwrap();
            }

            let window = app.get_window("main").unwrap();

            let mut context = Context::new();
            context.insert("version", "0.0.1");

            let window = app.get_window("main").unwrap();
            for (route, template) in routes.clone() {
                let route_clone = route.to_string();
                let template_clone = template.to_string();
                let window_clone = window.clone();
                let app_handle = app.handle();
                window.listen(format!("navigate/{}", route), move |_| {
                    let context = Context::new();
                    let rendered = TEMPLATES.render(&template_clone, &context).unwrap_or_else(|e| {
                        String::new()
                    });
                    window_clone.eval(&format!("history.pushState(null, '', '{}');", route_clone)).unwrap();
                    window_clone.eval(&format!("document.body.innerHTML = {:?}", rendered)).unwrap();

                    window_clone.eval("const styleSheet = document.createElement('style'); styleSheet.innerHTML = `{:?}`; document.head.appendChild(styleSheet);").unwrap();
                });
            }


            // Render the initial page
            let rendered = TEMPLATES.render("index.html", &context).unwrap_or_else(|e| {
                eprintln!("Error rendering template: {}", e);
                String::new()
            });
            window.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(1200, 1200))).unwrap();
            window.set_title("Bonemeal").unwrap();
            window.eval(&format!("document.body.innerHTML = {:?}", rendered)).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}