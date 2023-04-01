// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Machine ID
use machine_uid;

#[tauri::command]
fn machine_id() -> String {
    let id: String = machine_uid::get().unwrap();
    format!("{}", id.to_lowercase())
}

// Hash Key validation
use sha2::{Digest, Sha256};

#[tauri::command]
fn hash_string() -> String {
    let mut hasher = Sha256::new();
    let str = b"c15682a8-0eab-54ad-a902-4f43a69c72b8|6831453175974|2022-03-21T18:26:55.395Z";
    hasher.update(str);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![machine_id, hash_string])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
