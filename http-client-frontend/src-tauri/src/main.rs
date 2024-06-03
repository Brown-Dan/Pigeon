// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::iter::Map;
use reqwest;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: u16,
    size: String,
    body: String,
    headers: HashMap<String, String>
}

#[tauri::command]
async fn send_request(url: String) -> String {
    let response = reqwest::get(url).await;
    let response = match response {
        Ok(response) => response,
        Err(e) => return e.to_string(),
    };
    let status = response.status().as_u16();
    let headers = response.headers();

    let mut headers_map = HashMap::new();
    for (key, value) in headers.iter() {
        let header_value = value.to_str().unwrap().to_string();
        headers_map.insert(key.to_string(), header_value);
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return e.to_string(),
    };
    let size = body.len().to_string();
    let my_response = Response {
        status,
        body,
        size,
        headers: headers_map

    };
    return serde_json::to_string(&my_response).expect("Error");
}
