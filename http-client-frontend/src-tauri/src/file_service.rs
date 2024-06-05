use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddCollectionRequest {
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Collection {
    name: String,
    description: String,
    requests: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) method: String,
}

#[derive(Serialize, Deserialize)]
pub struct Requests {
    collections: Vec<Collection>,
    orphaned_requests: Vec<Request>,
}
#[derive(Serialize, Deserialize, Debug)]
struct History {
    requests: Vec<Request>
}

pub fn add_history(request: Request) {
    let mut path: PathBuf = get_pigeon_path();
    path.push("history.pigeon");

    let result = fs::read(&path);
    if result.is_err() {
        let mut requests: Vec<Request> = Vec::new();
        requests.push(request);
        let history: History = History {
            requests
        };
        let req: String = serde_json::to_string(&history).unwrap();
        fs::write(&path,  req).expect("Failed to create 'History.pigeon' file");
    } else {
        let req = String::from_utf8(result.unwrap()).expect("Invalid History Contents - Failure converting to String");
        let mut history: History = serde_json::from_str(&req).expect("Invalid History Contents");
        history.requests.push(request);
        fs::write(&path, serde_json::to_string(&history).unwrap()).expect("Failed to write updated history");
    }
}

pub fn add_collection(add_collection_request: AddCollectionRequest) -> bool {
    let mut path: PathBuf = get_pigeon_path();
    path.push(&add_collection_request.name);

    let create_dir_result = fs::create_dir(&path);
    if create_dir_result.is_err() {
        return false;
    }
    let contents: String = serde_json::to_string(&add_collection_request).unwrap();
    path.push("config.pigeon");
    let write_result = fs::write(&path, contents);
    if write_result.is_err() {
        return false;
    }
    return true;
}

pub fn get_files() -> Requests {
    let mut orphaned_requests: Vec<Request> = Vec::new();
    let mut collections: Vec<Collection> = Vec::new();

    let res = fs::read_dir(get_pigeon_path()).unwrap();
    for result in res {
        let entry: DirEntry = result.unwrap();
        if entry.file_type().unwrap().is_file() {
            if entry.file_name().eq("history.pigeon") {
                continue
            }
            let deserialized_req: Result<String, Error> = fs::read_to_string(entry.path());
            if deserialized_req.is_ok() {
                orphaned_requests.push(serde_json::from_str(deserialized_req.unwrap().as_str()).unwrap());
            }
        } else {
            let collection: Option<Collection> = get_collection(entry.path());
            if collection.is_some() {
                collections.push(collection.unwrap());
            }
        }
    }
    return Requests {
        collections,
        orphaned_requests,
    };
}

fn get_collection(path: PathBuf) -> Option<Collection> {
    let mut collection_config: Option<String> = None;
    let mut requests: Vec<String> = vec![];

    let result = fs::read_dir(path).unwrap();

    for item in result {
        let entry: DirEntry = item.unwrap();
        if entry.file_type().unwrap().is_file() {
            if entry.file_name().eq("config.pigeon") {
                collection_config = Some(fs::read_to_string(entry.path()).unwrap())
            } else {
                let deserialized_req: Result<String, Error> = fs::read_to_string(entry.path());
                if deserialized_req.is_ok() {
                    requests.push(fs::read_to_string(entry.path()).unwrap())
                }
            }
        }
    }
    return if collection_config.is_some() {
        Some(deserialize_collection(collection_config.unwrap(), requests))
    } else {
        None
    };
}

fn deserialize_collection(collection_config: String, requests: Vec<String>) -> Collection {
    let mut mapped_requests: Vec<Request> = vec![];
    requests.iter().for_each(|item| {
        mapped_requests.push(serde_json::from_str(item).unwrap());
    });
    let config: HashMap<String, String> = serde_json::from_str(&*collection_config).unwrap();

    return Collection {
        name: config.get("name").unwrap().to_string(),
        description: config.get("description").unwrap().to_string(),
        requests: mapped_requests,
    };
}

fn get_pigeon_path() -> PathBuf {
    let desktop_path: String = format!("{}/Desktop", dirs::home_dir().unwrap().to_string_lossy());
    let mut path_buf = PathBuf::new();
    path_buf.push(desktop_path);
    path_buf.push("Pigeon");
    return path_buf;
}
