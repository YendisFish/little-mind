use std::{fs};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn grab_config() -> String {
    let config = fs::read_to_string("../config.json");
    match config {
        Ok(config) => {
            println!("{}", config);
            config
        },
        Err(_) => "".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, grab_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
