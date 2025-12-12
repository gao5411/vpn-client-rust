use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct AuthRequest<'a> {
    user_id: &'a str,
    password: &'a str,
    device_code: &'a str,
    wg_public_key: &'a str,
}

#[tauri::command]
pub async fn authenticate(
    user_id: String,
    password: String,
    device_code: String,
    wg_public_key: String,
) -> Result<String, String> {
    let client = Client::new();
    // Replace with your actual server URL
    let url = "http://localhost:8080/api/auth/token"; 
    
    let resp = client.post(url)
        .json(&AuthRequest {
            user_id: &user_id,
            password: &password,
            device_code: &device_code,
            wg_public_key: &wg_public_key,
        })
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let data: serde_json::Value = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;

    if data["success"].as_bool().unwrap_or(false) {
        Ok(data["token"].as_str().unwrap_or("").to_string())
    } else {
        Err(data["msg"].as_str().unwrap_or("Authentication failed").to_string())
    }
}
