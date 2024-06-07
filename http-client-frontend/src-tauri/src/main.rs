// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::time::{Duration, Instant};

use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderName};
use serde::{Deserialize, Serialize};

use file_service::History;

use crate::file_service::{AddCollectionRequest, Request};

mod file_service;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_collections, send_request, add_collection, get_history, delete_collection, add_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: u16,
    size: String,
    body: String,
    headers: Vec<Header>,
    elapsed: Duration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryParam {
    name: String,
    value: String,
    enabled: bool,
}

fn vector_to_query_params(query_params: &Vec<QueryParam>) -> HashMap<&String, &String> {
    let mut query_params_map = HashMap::new();
    for query_param in query_params {
        if query_param.enabled {
            query_params_map.insert(&query_param.name, &query_param.value);
        }
    }
    return query_params_map;
}

fn vector_to_header_map(headers: &Vec<Header>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    for header in headers {
        let header_name: Result<HeaderName, InvalidHeaderName> = HeaderName::from_bytes(header.name.as_bytes());
        if header_name.is_err() || !header.enabled {
            continue;
        }
        header_map.append(header_name.unwrap(), HeaderValue::from_str(&*header.value).unwrap());
    }
    return header_map;
}

#[tauri::command]
async fn send_request(request: Request) -> String {
    let now = Instant::now();
    let response = reqwest::Client::new()
        .get(&request.url)
        .query(&vector_to_query_params(&request.query_params))
        .headers(vector_to_header_map(&request.headers))
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
fn get_collections() -> file_service::Requests {
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