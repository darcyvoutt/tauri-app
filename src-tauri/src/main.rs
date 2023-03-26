// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct RequestBody {
    username: String,
    age: u32,
    is_active: bool,
}

#[tauri::command]
async fn post_request(body: RequestBody) -> Result<Value, Error> {
    let response = reqwest::Client::new()
        .post("https://m32m1.mocklab.io/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let data: Value = response.json().await?;
    format!("Success: {:?}", data);

    Ok(().into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, post_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
