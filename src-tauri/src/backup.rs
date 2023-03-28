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
