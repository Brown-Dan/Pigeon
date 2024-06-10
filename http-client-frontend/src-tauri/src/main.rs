// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::model::{AddCollectionRequest, Header, History, QueryParam, Request, Requests, Response};

mod file_service;
mod request_service;
mod model;
mod request_service_test;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_collections, send_request, add_collection, get_history, delete_collection, add_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn map_query_param_vec_to_hashmap(query_params: &Vec<QueryParam>) -> HashMap<&String, &String> {
    return query_params.iter().filter_map(|item: &QueryParam| {
        match item.enabled {
            true => Some((&item.name, &item.value)),
            false => None
        }
    }).collect();
}

fn map_header_vec_to_hashmap(headers: &Vec<Header>) -> HeaderMap {
    headers.iter().filter_map(|item: &Header| {
        match item.enabled {
            true => Some((HeaderName::from_str(&item.name).unwrap(), HeaderValue::from_str(&*item.value).unwrap())),
            false => None
        }
    }).collect()
}

#[tauri::command]
async fn send_request(request: Request) -> String {
    let now = Instant::now();
    let response = reqwest::Client::new()
        .get(&request.url)
        .query(&map_query_param_vec_to_hashmap(&request.query_params))
        .headers(map_header_vec_to_hashmap(&request.headers))
        .send().await;
    let elapsed = now.elapsed();
    let response = match response {
        Ok(response) => response,
        Err(e) => return e.to_string(),
    };
    let status = response.status().as_u16();
    let response_headers = response.headers();

    let mut response_headers_vec: Vec<Header> = Vec::new();
    for (key, value) in response_headers.iter() {
        let header_value = value.to_str().unwrap().to_string();
        response_headers_vec.push(Header { name: key.to_string(), value: header_value, enabled: false });
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
        headers: response_headers_vec,
        elapsed,
    };
    let historic_request: Request = Request {
        name: String::from("_"),
        url: request.url,
        method: request.method,
        collection_name: String::from("_"),
        headers: request.headers,
        query_params: request.query_params,
    };
    file_service::add_history(historic_request, &my_response);

    return serde_json::to_string(&my_response).expect("Error");
}

#[tauri::command]
fn get_collections() -> Requests {
    return file_service::get_files();
}

#[tauri::command]
fn add_collection(config: AddCollectionRequest) -> bool {
    return file_service::add_collection(config);
}

#[tauri::command]
fn get_history() -> History {
    return file_service::get_history();
}

#[tauri::command]
fn delete_collection(collection_name: String) {
    file_service::delete_collection(collection_name);
}

#[tauri::command]
fn add_request(request: Request) {
    file_service::add_request(request);
}