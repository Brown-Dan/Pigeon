// use std::collections::HashMap;
// use std::str::FromStr;
// use std::time::Instant;
// use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
// use crate::file_service;
// use crate::model::{Header, QueryParam, Request, RequestMethod, Response};
//
// pub fn send_request(request: Request) -> Response {
//     match(&request.method) {
//         RequestMethod::GET => get(request),
//         RequestMethod::POST => post(request),
//         RequestMethod::DELETE => post(request),
//         RequestMethod::PATCH => post(request),
//         RequestMethod::PUT => post(request)
//
//     }
// }
//
// fn add_historic_request(request: Request, response: Response) {
//     file_service::add_history(request, &response);
// }
//
//
// async fn get(request: Request) -> Response {
//     let now = Instant::now();
//     let response = reqwest::Client::new()
//         .get(&request.url)
//         .query(&crate::map_query_param_vec_to_hashmap(&request.query_params))
//         .headers(crate::map_header_vec_to_hashmap(&request.headers))
//         .send().await;
//     let elapsed = now.elapsed();
// }
//
// fn post(request: Request) -> Response {
//         // TODO needs request body
// }
//
// async fn delete(request: Request) -> Response {
//     let now = Instant::now();
//     let response = reqwest::Client::new()
//         .delete(&request.url)
//         .query(&crate::map_query_param_vec_to_hashmap(&request.query_params))
//         .headers(crate::map_header_vec_to_hashmap(&request.headers))
//         .send().await;
//     let elapsed = now.elapsed();
// }
//
// fn patch(request: Request) -> Response {
//     // TODO needs request body
// }
//
// fn put(request: Request) -> Response {
//     // TODO needs request body
// }
//
// fn map_query_param_vec_to_hashmap(query_params: &Vec<QueryParam>) -> HashMap<&String, &String> {
//     return query_params.iter().map( |item: &QueryParam|  (&item.name, &item.value) ).collect();
// }
//
// fn map_header_vec_to_hashmap(headers: &Vec<Header>) -> HeaderMap {
//     headers.iter().filter_map(|item: &Header| {
//         match item.enabled {
//             true => Some((HeaderName::from_str(&item.name).unwrap(),  HeaderValue::from_str(&*item.value).unwrap())),
//             false => None
//         }
//     }).collect()
// }