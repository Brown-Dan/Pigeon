use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;

use reqwest::header::{HeaderMap, HeaderName};

use crate::file_service;
use crate::model::{Header, QueryParam, Request, RequestMethod, Response};

lazy_static! {
    static ref CLIENT: Arc<reqwest::Client> = Arc::new(reqwest::Client::new());
}

fn get_client() -> Arc<reqwest::Client> {
    Arc::clone(&CLIENT)
}

pub async fn send_request(request: Request) -> Option<Response> {
    let now = Instant::now();

    let request_to_send = {
        let req = get_client()
            .request(request.get_reqwest_method(), &request.url)
            .query(&map_query_param_vec_to_hashmap(&request.query_params))
            .headers(map_header_vec_to_header_map(&request.headers))
            .timeout(Duration::from_secs(30));

        if request.body.enabled {
            req.body(request.body.content.clone())
        } else {
            req
        }
    };

    let result = request_to_send.send().await;
    let elapsed = now.elapsed();

    let result: Option<Response> = async {
        match result {
            Ok(response) => Some(map_response(response, elapsed).await),
            Err(_) => None,
        }
    }.await;

    result.inspect(|response| file_service::add_history(request, &response))
}

async fn map_response(response: reqwest::Response, duration: Duration) -> Response {
    let headers = map_header_map_to_header_vec(response.headers());
    let content_type: Option<String> = headers
        .iter()
        .find(|item| item.name.eq("content-type"))
        .map(|item| item.value.clone());
    let status = response.status().as_u16();
    let body = &response.text().await.unwrap_or_else(|e| e.to_string());
    Response {
        status,
        body: body.clone(),
        size: body.len().to_string().clone(),
        headers,
        elapsed: duration,
        content_type: content_type.unwrap().clone(),
    }
}

fn map_header_map_to_header_vec(headers: &HeaderMap) -> Vec<Header> {
    headers
        .iter()
        .map(|(key, value)| Header {
            name: key.to_string(),
            value: value.to_str().unwrap().to_string(),
            enabled: true,
        })
        .collect()
}

fn map_query_param_vec_to_hashmap(query_params: &Vec<QueryParam>) -> HashMap<String, String> {
    query_params
        .iter()
        .filter(|item| item.enabled)
        .map(|item| (item.name.clone(), item.value.clone()))
        .collect()
}

fn map_header_vec_to_header_map(headers: &Vec<Header>) -> HeaderMap {
    headers
        .iter()
        .filter(|item| item.enabled && !item.name.is_empty())
        .map(|item| (HeaderName::from_str(&item.name).unwrap(), item.value.parse().unwrap()))
        .collect()
}

impl Request {
    pub fn get_reqwest_method(&self) -> reqwest::Method {
        match &self.method {
            RequestMethod::GET => reqwest::Method::GET,
            RequestMethod::POST => reqwest::Method::POST,
            RequestMethod::DELETE => reqwest::Method::DELETE,
            RequestMethod::PATCH => reqwest::Method::PATCH,
            RequestMethod::PUT => reqwest::Method::PUT,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use httpmock::Method::PATCH;

    use httpmock::prelude::*;
    use reqwest::header::{ACCEPT, HeaderMap};
    use tokio;

    use crate::model::{Body, Header, QueryParam, Request, RequestMethod};
    use crate::request_service::{map_header_map_to_header_vec, map_header_vec_to_header_map, map_query_param_vec_to_hashmap, send_request};

    #[tokio::test]
    async fn send_request_given_put_request_should_return_serialized_response() {
        let server = MockServer::start();
        let request = Request {
            name: String::from("PUT - Test"),
            url: server.url("/test"),
            method: RequestMethod::PUT,
            collection_name: String::from("Test Collection"),
            headers: vec![Header { name: String::from("test_header"), value: String::from("test"), enabled: true }],
            query_params: vec![QueryParam { name: String::from("test_query_param"), value: String::from("test"), enabled: true }],
            body: Body {
                content: String::from("{\"key\" : \"value\"}"),
                enabled: false,
            },
        };
        let mock = server.mock(|when, then| {
            when.method(PUT)
                .header("test_header", "test")
                .query_param("test_query_param", "test")
                .path("/test");
            then.status(205)
                .header("content-type", "text/html")
                .header("date", "2024-10-10")
                .body("Hey!");
        });

        let result = send_request(request).await;

        mock.assert();
        match result {
            Some(res) => {
                assert_eq!(res.status, 205);
                assert_eq!(res.size, "4");
                assert_eq!(res.body, "Hey!");
                assert_eq!(res.headers, vec![
                    Header { name: String::from("content-type"), value: String::from("text/html"), enabled: true },
                    Header { name: String::from("date"), value: String::from("2024-10-10"), enabled: true },
                    Header { name: String::from("content-length"), value: String::from("4"), enabled: true },
                ]);
                assert_eq!(res.content_type, "text/html");
            }
            None => assert!(false, "Optional value is None")
        }
    }

    #[tokio::test]
    async fn send_request_given_patch_request_should_return_serialized_response() {
        let server = MockServer::start();
        let request = Request {
            name: String::from("PATCH - Test"),
            url: server.url("/test"),
            method: RequestMethod::PATCH,
            collection_name: String::from("Test Collection"),
            headers: vec![Header { name: String::from("test_header"), value: String::from("test"), enabled: true }],
            query_params: vec![QueryParam { name: String::from("test_query_param"), value: String::from("test"), enabled: true }],
            body: Body {
                content: String::from("{\"key\" : \"value\"}"),
                enabled: true,
            },
        };
        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .header("test_header", "test")
                .query_param("test_query_param", "test")
                .body("{\"key\" : \"value\"}")
                .path("/test");
            then.status(400)
                .header("content-type", "text/html")
                .header("date", "2024-10-10")
                .body("Hey!");
        });

        let result = send_request(request).await;

        mock.assert();
        match result {
            Some(res) => {
                assert_eq!(res.status, 400);
                assert_eq!(res.size, "4");
                assert_eq!(res.body, "Hey!");
                assert_eq!(res.headers, vec![
                    Header { name: String::from("content-type"), value: String::from("text/html"), enabled: true },
                    Header { name: String::from("date"), value: String::from("2024-10-10"), enabled: true },
                    Header { name: String::from("content-length"), value: String::from("4"), enabled: true },
                ]);
                assert_eq!(res.content_type, "text/html");
            }
            None => assert!(false, "Optional value is None")
        }
    }

    #[tokio::test]
    async fn send_request_given_delete_request_should_return_serialized_response() {
        let server = MockServer::start();
        let request = Request {
            name: String::from("DELETE - Test"),
            url: server.url("/test"),
            method: RequestMethod::DELETE,
            collection_name: String::from("Test Collection"),
            headers: vec![Header { name: String::from("test_header"), value: String::from("test"), enabled: true }],
            query_params: vec![QueryParam { name: String::from("test_query_param"), value: String::from("test"), enabled: true }],
            body: Body {
                content: String::from("{\"key\" : \"value\"}"),
                enabled: false,
            },
        };
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .header("test_header", "test")
                .query_param("test_query_param", "test")
                .path("/test");
            then.status(202)
                .header("content-type", "text/html")
                .header("date", "2024-10-10")
                .body("Hey!");
        });

        let result = send_request(request).await;

        mock.assert();
        match result {
            Some(res) => {
                assert_eq!(res.status, 202);
                assert_eq!(res.size, "4");
                assert_eq!(res.body, "Hey!");
                assert_eq!(res.headers, vec![
                    Header { name: String::from("content-type"), value: String::from("text/html"), enabled: true },
                    Header { name: String::from("date"), value: String::from("2024-10-10"), enabled: true },
                    Header { name: String::from("content-length"), value: String::from("4"), enabled: true },
                ]);
                assert_eq!(res.content_type, "text/html");
            }
            None => assert!(false, "Optional value is None")
        }
    }

    #[tokio::test]
    async fn send_request_given_post_request_should_return_serialized_response() {
        let server = MockServer::start();
        let request = Request {
            name: String::from("POST - Test"),
            url: server.url("/test"),
            method: RequestMethod::POST,
            collection_name: String::from("Test Collection"),
            headers: vec![Header { name: String::from("test_header"), value: String::from("test"), enabled: true }],
            query_params: vec![QueryParam { name: String::from("test_query_param"), value: String::from("test"), enabled: true }],
            body: Body {
                content: String::from("{\"key\" : \"value\"}"),
                enabled: true,
            },
        };
        let mock = server.mock(|when, then| {
            when.method(POST)
                .header("test_header", "test")
                .query_param("test_query_param", "test")
                .body("{\"key\" : \"value\"}")
                .path("/test");
            then.status(201)
                .header("content-type", "text/html")
                .header("date", "2024-10-10")
                .body("Hey!");
        });

        let result = send_request(request).await;

        mock.assert();
        match result {
            Some(res) => {
                assert_eq!(res.status, 201);
                assert_eq!(res.size, "4");
                assert_eq!(res.body, "Hey!");
                assert_eq!(res.headers, vec![
                    Header { name: String::from("content-type"), value: String::from("text/html"), enabled: true },
                    Header { name: String::from("date"), value: String::from("2024-10-10"), enabled: true },
                    Header { name: String::from("content-length"), value: String::from("4"), enabled: true },
                ]);
                assert_eq!(res.content_type, "text/html");
            }
            None => assert!(false, "Optional value is None")
        }
    }

    #[tokio::test]
    async fn send_request_given_get_request_should_return_serialized_response() {
        let server = MockServer::start();
        let request = Request {
            name: String::from("GET - Test"),
            url: server.url("/test"),
            method: RequestMethod::GET,
            collection_name: String::from("Test Collection"),
            headers: vec![Header { name: String::from("test_header"), value: String::from("test"), enabled: true }],
            query_params: vec![QueryParam { name: String::from("test_query_param"), value: String::from("test"), enabled: true }],
            body: Body {
                content: String::from(""),
                enabled: false,
            },
        };
        let mock = server.mock(|when, then| {
            when.method(GET)
                .header("test_header", "test")
                .query_param("test_query_param", "test")
                .path("/test");
            then.status(200)
                .header("content-type", "text/html")
                .header("date", "2024-10-10")
                .body("Hey!");
        });

        let result = send_request(request).await;

        mock.assert();
        match result {
            Some(res) => {
                assert_eq!(res.status, 200);
                assert_eq!(res.size, "4");
                assert_eq!(res.body, "Hey!");
                assert_eq!(res.headers, vec![
                    Header { name: String::from("content-type"), value: String::from("text/html"), enabled: true },
                    Header { name: String::from("date"), value: String::from("2024-10-10"), enabled: true },
                    Header { name: String::from("content-length"), value: String::from("4"), enabled: true },
                ]);
                assert_eq!(res.content_type, "text/html");
            }
            None => assert!(false, "Optional value is None")
        }
    }

    #[test]
    fn map_header_map_to_header_vec_given_empty_header_map_should_return_empty_headers() {
        let result = map_header_map_to_header_vec(&HeaderMap::new());

        assert_eq!(result.len(), 0)
    }

    #[test]
    fn map_header_map_to_header_vec_given_header_map_should_return_headers() {
        let mut input = HeaderMap::new();
        input.append(ACCEPT, "*/*".parse().unwrap());

        let expected = vec![Header {
            name: String::from("accept"),
            value: String::from("*/*"),
            enabled: true,
        }];

        let result = map_header_map_to_header_vec(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn map_query_param_vec_to_hashmap_given_no_query_params_should_empty_hashmap() {
        let result = map_query_param_vec_to_hashmap(&Vec::new());

        assert_eq!(result.len(), 0)
    }

    #[test]
    fn map_query_param_vec_to_hashmap_given_query_params_should_return_hashmap() {
        let included_query_param = QueryParam {
            name: String::from("included"),
            value: String::from("included"),
            enabled: true,
        };

        let excluded_query_param = QueryParam {
            name: String::from("excluded"),
            value: String::from("excluded"),
            enabled: false,
        };

        let expected = HashMap::from([(String::from("included"), String::from("included"))]);

        let result = map_query_param_vec_to_hashmap(&vec![included_query_param, excluded_query_param]);

        assert_eq!(result, expected)
    }

    #[test]
    fn map_header_vec_to_header_map_given_headers_should_return_header_map() {
        let included_header = Header {
            name: String::from("accept"),
            value: String::from("*/*"),
            enabled: true,
        };
        let excluded_header = Header {
            name: String::from("host"),
            value: String::from("exclude.this.header"),
            enabled: false,
        };

        let mut expected = HeaderMap::new();
        expected.append(ACCEPT, "*/*".parse().unwrap());

        let result = map_header_vec_to_header_map(&vec![included_header, excluded_header]);

        assert_eq!(result, expected);
    }

    #[test]
    fn map_header_vec_to_header_map_given_no_headers_should_return_empty_header_map() {
        let result = map_header_vec_to_header_map(&Vec::new());

        assert_eq!(result.len(), 0);
    }
}
