// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::model::{AddCollectionRequest, History, Request, Requests, Response};

mod file_service;
mod request_service;
mod model;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_collections, send_request, add_collection, get_history, delete_collection, add_request, delete_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn send_request(request: Request) -> String {
    let response = request_service::send_request(request).await;
    match &response {
        Some(r) => serde_json::to_string(r).unwrap(),
        None => String::from("Error sending Request"),
    }
}

#[tauri::command]
fn get_collections() -> Requests {
    file_service::get_files()
}

#[tauri::command]
fn add_collection(config: AddCollectionRequest) -> bool {
    file_service::add_collection(config)
}

#[tauri::command]
fn get_history() -> History {
    file_service::get_history()
}

#[tauri::command]
fn delete_collection(collection_name: String) {
    file_service::delete_collection(collection_name);
}

#[tauri::command]
fn add_request(request: Request) {
    file_service::add_request(request);
}

#[tauri::command]
fn delete_request(request: Request) {
    file_service::delete_request(request);
}
