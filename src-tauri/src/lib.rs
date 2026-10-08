use std::fs;
use redb::Database;

use crate::db::{create_chat, get_chat_ids};

mod db;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn grab_config() -> String {
    let config = fs::read_to_string("../config.json");
    match config {
        Ok(config) => config,
        Err(_) => "".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::create("../db.redb").unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            grab_config,
            create_chat,
            get_chat_ids,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
