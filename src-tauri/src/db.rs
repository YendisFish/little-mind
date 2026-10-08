use redb::{Database, Range, ReadableDatabase, ReadableTable, TableDefinition};
use tauri::State;
use uuid::{Timestamp, Uuid};

pub const CHAT_TABLE: TableDefinition<&str, Vec<String>> = TableDefinition::new("chat");

#[tauri::command]
pub fn get_chat_ids(db: State<'_, Database>) -> Vec<String> {
    println!("actually getting stuff");
    let Some(ta) = db.begin_read().ok() else { return Vec::new() };
    let Some(tbl) = ta.open_table(CHAT_TABLE).ok() else { return Vec::new() };

    let mut ret = Vec::new();
    for kv in tbl.iter().unwrap() {
        let elem = kv.unwrap();
        println!("{}", elem.0.value());
        ret.push(elem.0.value().to_string());
    }

    ret
}

#[tauri::command]
pub fn create_chat(db: State<'_, Database>) -> String {
    let uid = Uuid::now_v7();

    let Some(ta) = db.begin_write().ok() else { return String::new() };

    {
        let Some(mut tbl) = ta.open_table(CHAT_TABLE).ok() else { return String::new() };
        _ = tbl.insert(uid.to_string().as_str(), Vec::new());
    }

    _ = ta.commit();
    return uid.to_string();
}
