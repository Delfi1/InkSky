#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;

use std::sync::OnceLock;
use tauri::{AppHandle, Manager, RunEvent, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn main_window() -> WebviewWindow {
    APP_HANDLE.get().unwrap()
        .get_webview_window("main")
        .expect("Get window error")
}

#[tauri::command]
fn register(name: String, email: String, password: String) -> String {
    println!("Registration: {}, {}: {}...", email, name, password);

    network::register(email, name, password)
}

#[tauri::command]
fn is_email_taken(email: String) -> bool {
    network::is_email_taken(&email)
}

#[tauri::command]
fn login(email: String, password: String){
    network::login(email, password)
}

#[tauri::command]
fn debug(content: String) {
    println!("Debug: {}", content);
}

#[tauri::command]
fn error(content: String) {
    eprintln!("Error: {}", content)
}

fn init_window(handle: &AppHandle) {
    let builder = WebviewWindowBuilder::new(handle, "main", WebviewUrl::App("index.html".into()))
        .inner_size(1280.0, 720.0)
        .min_inner_size(640.0, 450.0)
        .focused(true);

    let mut title = format!("InkSky v{}", env!("CARGO_PKG_VERSION"));
    #[cfg(debug_assertions)]
    { title += "dev" }

    #[cfg(debug_assertions)]
    let builder = builder.always_on_top(true);

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
        .invoke_handler(tauri::generate_handler![register, login, debug, error, is_email_taken])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|handle, event| match event {
        RunEvent::Ready => {
            network::init();
            init_window(handle);
            let app_handle = handle.clone();
            APP_HANDLE.get_or_init(|| { app_handle });
            println!("Ready!");
        },
        _ => ()
    })
}
