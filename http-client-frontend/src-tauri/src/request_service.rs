use std::collections::HashMap;
use std::str::FromStr;
use std::time::{Duration, Instant};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::file_service;
use crate::model::{Header, QueryParam, Request, RequestMethod, Response};

pub async fn send_request(request: Request) -> String {
    let now = Instant::now();
    let result = match &request.method {
        RequestMethod::GET => get(&request).await,
        RequestMethod::POST => post(&request).await,
        RequestMethod::DELETE => post(&request).await,
        RequestMethod::PATCH => post(&request).await,
        RequestMethod::PUT => post(&request).await
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


async fn get(request: &Request) -> Result<reqwest::Response, reqwest::Error> {
    let response = reqwest::Client::new()
        .get(&request.url)
        .query(&map_query_param_vec_to_hashmap(&request.query_params))
        .headers(map_header_vec_to_header_map(&request.headers))
        .send().await;
    return response;
}

async fn post(request: &Request) -> Result<reqwest::Response, reqwest::Error> {
    let sent = Instant::now();
    let response = reqwest::Client::new()
        .post(&request.url)
        .body(request.body.content.clone())
        .query(&map_query_param_vec_to_hashmap(&request.query_params))
        .headers(map_header_vec_to_header_map(&request.headers))
        .send().await;
    let elapsed = sent.elapsed();
    return response;
}

async fn delete(request: &Request) -> Result<reqwest::Response, reqwest::Error> {
    let response = reqwest::Client::new()
        .delete(&request.url)
        .query(&crate::map_query_param_vec_to_hashmap(&request.query_params))
        .headers(crate::map_header_vec_to_hashmap(&request.headers))
        .send().await;
    return response;
}

async fn patch(request: &Request) -> Result<reqwest::Response, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(&request.url)
        // .body(&*request.body.content)
        .query(&map_query_param_vec_to_hashmap(&request.query_params))
        .headers(map_header_vec_to_header_map(&request.headers))
        .send().await;
    return response;
}

async fn put(request: &Request) -> Result<reqwest::Response, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(&request.url)
        // .body(&*request.body.content)
        .query(&map_query_param_vec_to_hashmap(&request.query_params))
        .headers(map_header_vec_to_header_map(&request.headers))
        .send().await;
    return response;
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