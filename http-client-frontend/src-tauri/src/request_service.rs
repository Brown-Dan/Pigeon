use std::collections::HashMap;
use std::str::FromStr;
use std::time::{Duration, Instant};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use tauri::http::ResponseBuilder;

use crate::file_service;
use crate::model::{Header, QueryParam, Request, RequestMethod, Response};

pub async fn send_request(request: Request) -> String {
    let now = Instant::now();
    let result = match &request.method {
        RequestMethod::GET =>  build_request(&request, get(&request)).send().await,
        RequestMethod::POST => build_request(&request, post(&request)).send().await,
        RequestMethod::DELETE => build_request(&request, delete(&request)).send().await,
        RequestMethod::PATCH => build_request(&request, patch(&request)).send().await,
        RequestMethod::PUT => build_request(&request, put(&request)).send().await
    };
    let elapsed = now.elapsed();
    let response = match result {
        Ok(response) => response,
        Err(e) => return e.to_string(),
    };

    let mapped_response = map_response(response, elapsed).await;

    file_service::add_history(request, &mapped_response);
    return serde_json::to_string(&mapped_response).expect("Error");
}

fn build_request(request: &Request, request_builder: RequestBuilder) -> RequestBuilder {
    let mut request_builder = request_builder
        .query(&map_query_param_vec_to_hashmap(&request.query_params))
        .headers(map_header_vec_to_header_map(&request.headers));
    if (request.body.enabled) {
        request_builder = request_builder.body(request.body.content.clone());
    }
    return request_builder;
}

fn add_historic_request(request: Request, response: Response) {
    file_service::add_history(request, &response);
}

async fn map_response(response: reqwest::Response, duration: Duration) -> Response {
    let headers = map_header_map_to_header_vec(response.headers());
    let content_type: Option<String> = headers.iter().find(|item| item.name == "content-type").map(|item| item.value.clone());
    let status = response.status().as_u16();
    let body = &response.text().await.unwrap_or_else(|e| e.to_string());
    return Response {
        status,
        body: body.clone(),
        size: body.len().to_string().clone(),
        headers,
        elapsed: duration,
        content_type: content_type.unwrap().clone(),
    };
}

fn get(request: &Request) -> RequestBuilder {
    return reqwest::Client::new().get(&request.url);
}

fn post(request: &Request) -> RequestBuilder {
     return reqwest::Client::new().post(&request.url);
}

fn delete(request: &Request) -> RequestBuilder {
    return reqwest::Client::new().delete(&request.url);
}

fn patch(request: &Request) -> RequestBuilder {
    return reqwest::Client::new().patch(&request.url);
}

fn put(request: &Request) -> RequestBuilder {
    return reqwest::Client::new().put(&request.url);
}

fn map_header_map_to_header_vec(headers: &HeaderMap) -> Vec<Header> {
    // TODO refactor
    let mut response_headers_vec: Vec<Header> = Vec::new();
    for (key, value) in headers.iter() {
        let header_value = value.to_str().unwrap().to_string();
        response_headers_vec.push(Header { name: key.to_string(), value: header_value, enabled: false });
    }
    return response_headers_vec;
}

pub fn map_query_param_vec_to_hashmap(query_params: &Vec<QueryParam>) -> HashMap<&String, &String> {
    return query_params.iter().filter_map(|item: &QueryParam| {
        match item.enabled {
            true => Some((&item.name, &item.value)),
            false => None
        }
    }).collect();
}

pub fn map_header_vec_to_header_map(headers: &Vec<Header>) -> HeaderMap {
    headers.iter().filter_map(|item: &Header| {
        match item.enabled {
            true => Some((HeaderName::from_str(&item.name).unwrap(), HeaderValue::from_str(&*item.value).unwrap())),
            false => None
        }
    }).collect()
}