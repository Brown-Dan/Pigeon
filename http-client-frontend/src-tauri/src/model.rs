use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) method: RequestMethod,
    pub(crate) collection_name: String,
    pub(crate) headers: Vec<Header>,
    pub(crate) query_params: Vec<QueryParam>, 
    pub(crate) body: Body
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    pub(crate) content: String,
    pub(crate) enabled: bool
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Response {
    pub(crate) status: u16,
    pub(crate) size: String,
    pub(crate) body: String,
    pub(crate) headers: Vec<Header>,
    pub(crate) elapsed: Duration,
    pub(crate) content_type: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Header {
    pub(crate) name: String,
    pub(crate) value: String,
    pub(crate) enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryParam {
    pub(crate) name: String,
    pub(crate) value: String,
    pub(crate) enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AddCollectionRequest {
    pub(crate) name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) requests: Vec<Request>,
}

#[derive(Serialize, Deserialize)]
pub struct Requests {
    pub(crate) collections: Vec<Collection>,
    pub(crate) orphaned_requests: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    pub(crate) requests: Vec<HistoricRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricRequest {
    pub(crate) time: SystemTime,
    pub(crate) url: String,
    pub(crate) method: RequestMethod,
    pub(crate) response_status: u16,
    pub(crate) size: String,
    pub(crate) speed: Duration,
}
