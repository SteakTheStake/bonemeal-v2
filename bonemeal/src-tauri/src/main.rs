// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;
extern crate tera;

use std::collections::HashMap;
use tauri::Manager;
use tera::{Context, Tera};
use window_shadows::set_shadow;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("../../ui/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}


fn do_nothing_filter(_value: &serde_json::Value, _args: &HashMap<String, serde_json::Value>) -> tera::Result<serde_json::Value> {
    // This is just a placeholder filter that does nothing.
    // You can replace it with your own filter implementation if needed.
    Ok(serde_json::Value::Null)
}

fn main() {
    let tera = Tera::new("../../ui/**/*.html").unwrap();

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

            let mut context = Context::new();
            context.insert("version", "0.0.1");

            let window = app.get_window("main").unwrap();
            for (route, template) in routes.clone() {
                let route_clone = route.to_string();
                let template_clone = template.to_string();
                let tera_clone = tera.clone();
                let context_clone = context.clone();
                let window_clone = window.clone();
                window.listen(format!("navigate/{}", route), move |_| {
                    let tera = tera_clone.clone();
                    let context = context_clone.clone();
                    let window = window_clone.clone();
                    let rendered = tera.render(&template_clone, &context).unwrap();
                    window.eval(&format!("history.pushState(null, '', '{}');", route_clone)).unwrap();
                    window.eval(&format!("document.body.innerHTML = {:?}", rendered)).unwrap();
                });
            }
            // Render the initial page
            let rendered = tera.render("index.html", &context).unwrap();
            window.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(1200, 800))).unwrap();
            window.set_title("Bonemeal").unwrap();
            window.eval(&format!("document.body.innerHTML = {:?}", rendered)).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}