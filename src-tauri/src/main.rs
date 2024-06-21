#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;
use std::sync::OnceLock;
use tauri::{AppHandle, Manager, RunEvent, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[tauri::command]
fn debug(content: String) {
    println!("Debug: {}", content);
}

#[tauri::command]
fn error(content: String) {
    eprintln!("Error: {}", content)
}

#[tauri::command]
fn connect() {
    network::reconnect();
}

#[tauri::command]
fn is_login() -> bool {
    network::is_login()
}

fn init_window(handle: &AppHandle) {
    let builder = WebviewWindowBuilder::new(handle, "main", WebviewUrl::App("index.html".into()))
        .inner_size(640.0, 680.0)
        .min_inner_size(640.0, 640.0)
        .focused(true);

    let mut title = format!("InkSky v{}", env!("CARGO_PKG_VERSION"));
    #[cfg(debug_assertions)]
    let builder = {
        title += "dev";
        builder.always_on_top(true)
    };

    let builder = builder.title(title);

    let window = builder.build()
        .expect("Main window init error");

    #[cfg(debug_assertions)]
    window.open_devtools();
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![connect, is_login, debug, error])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|handle, event| match event {
        RunEvent::Ready => {
            init_window(handle);
            println!("Ready!");
        },
        _ => ()
    })
}
