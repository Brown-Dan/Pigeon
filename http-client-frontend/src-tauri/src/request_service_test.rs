use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use crate::model::Header;
use crate::request_service::{map_header_vec_to_header_map};

#[test]
fn map_header_vec_to_header_map__given_headers__should_return_header_map() {
    let included_header = Header {
        name: String::from("included"),
        value: String::from("included"),
        enabled: true
    };
    let excluded_header = Header {
        name: String::from("excluded"),
        value: String::from("excluded"),
        enabled: false
    };

    let mut input: Vec<Header> = Vec::new();
    input.push(included_header);
    input.push(excluded_header);

    let mut expected = HeaderMap::new();
    expected.append(HeaderName::from_str("included").unwrap(),
                    HeaderValue::from_str("included").unwrap());

    let result = map_header_vec_to_header_map(&input);
    assert_eq!(result, expected);
}

#[test]
fn map_header_vec_to_header_map__given_no_headers__should_return_empty_header_map() {
    let input: Vec<Header> = Vec::new();
    let expected = HeaderMap::new();

    let result = map_header_vec_to_header_map(&input);
    assert_eq!(result, expected);
}