use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::model::{AddCollectionRequest, Collection, HistoricRequest, History, Request, Requests};
use crate::Response;

fn get_history_path() -> PathBuf {
    let mut path: PathBuf = get_pigeon_path();
    path.push("history.pigeon");
    return path;
}

pub fn get_history() -> History {
    fs::read(&get_history_path())
        .map_or(History { requests: Vec::new() },
                |history_as_bytes: Vec<u8>| {
                    let mut history: History = serde_json::from_str(&*String::from_utf8(history_as_bytes).unwrap()).unwrap();
                    history.requests.reverse();
                    return history;
                })
}

fn build_historic_request(request: &Request, response: &Response) -> HistoricRequest {
    return HistoricRequest {
        time: SystemTime::now(),
        url: request.url.clone(),
        method: request.method.clone(),
        response_status: response.status,
        size: response.size.clone(),
        speed: response.elapsed,
    };
}

pub fn add_history(request: Request, response: &Response) {
    let history_path: PathBuf = get_history_path();
    fs::write(&history_path, serde_json::to_string(&fs::read(&history_path)
        .map_or(History {
            requests: {
                let mut requests: Vec<HistoricRequest> = Vec::new();
                requests.push(build_historic_request(&request, response));
                requests
            }
        }, |item: Vec<u8>| {
            let mut history: History = serde_json::from_str(&*String::from_utf8(item).unwrap()).unwrap();
            history.requests.push(build_historic_request(&request, response));
            return history;
        })).unwrap()).unwrap();
}

pub fn delete_collection(collection_name: String) {
    let mut path: PathBuf = get_pigeon_path();
    path.push(&collection_name);
    fs::remove_dir_all(path).unwrap()
}

pub fn add_request(request: Request) {
    let mut path: PathBuf = get_pigeon_path();
    if request.collection_name.ne("orphan") { path.push(&request.collection_name); }
    let pigeon_ext: String = String::from(".pigeon");
    let mut file: String = request.name.clone();
    file.push_str(&pigeon_ext);
    path.push(file);
    fs::write(&path, serde_json::to_string(&request).unwrap()).unwrap()
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
            if entry.file_name().eq("history.pigeon") || entry.file_name().eq("scratchpad.pigeon") {
                continue;
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

fn delete_history() {
    let path = get_history_path();
    fs::write(path, "").unwrap();
}