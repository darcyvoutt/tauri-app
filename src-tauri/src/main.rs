// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Machine ID
use machine_uid;

#[tauri::command]
fn machine_id() -> String {
    let id: String = machine_uid::get().unwrap();
    format!("{}", id.to_lowercase())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![machine_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
