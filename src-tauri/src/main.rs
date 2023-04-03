// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Get Env Variable
use dotenv::dotenv;

fn get_secret() -> String {
    if cfg!(debug_assertions) {
        dotenv().ok();
        let secret_string = std::env::var("SECRET").expect("Environment variable not set.");
        return secret_string;
    } else {
        return option_env!("SECRET").unwrap_or_default().to_owned();
    }
}

// Machine ID
use machine_uid;

#[tauri::command]
fn machine_id() -> String {
    let id: String = machine_uid::get().unwrap();
    format!("{}", id.to_lowercase())
}

// Hash Key validation
use sha2::{Digest, Sha256};
use std::env;

#[tauri::command]
fn hash_string(customer_id: &str) -> String {
    // Use machine ID
    let machine: String = String::from(machine_id());

    // Get Secret from ENV Variables
    let secret_string = get_secret();
    let secret: &str = &secret_string[..];

    println!("the secret key might be: {secret:?}");

    // Customer ID
    let customer_id: String = String::from(customer_id);
    let customer: &str = &customer_id[..];

    println!("the input setting is: {customer:?}");

    let key: String = machine + "|" + secret + "|" + customer;
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
