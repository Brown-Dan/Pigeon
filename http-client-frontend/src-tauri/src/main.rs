// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![send_request])
      .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn send_request(url: String) -> String {
  let response = reqwest::get(url).await;
  let response = match response {
    Ok(response) => response,
    Err(e) => return e.to_string(),
  };
  return response.text().await.expect("There was an error.");
}
