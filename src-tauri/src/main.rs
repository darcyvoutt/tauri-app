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
use dotenv::dotenv;
use sha2::{Digest, Sha256};

#[tauri::command]
fn hash_string() -> String {
    dotenv().ok();

    // Get Secret from ENV Variables
    let secret: String = std::env::var("SECRET").expect("$SECRET is not set");
    let secret_str: &str = &secret[..];

    println!("{}", secret_str);

    // Get other variables
    let machine: String = String::from(machine_id());
    let customer: &str = "6831453175974";
    let timestamp: &str = "2022-03-21T18:26:55.395Z";

    let key: String = machine + "|" + customer + "|" + timestamp + "|" + secret_str;
    let mut hash = Sha256::new();
    hash.update(key);
    let hashed = hash.finalize();
    format!("{:x}", hashed)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![machine_id, hash_string])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
