use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;
#[allow(unused_imports)]
use std::time::SystemTime;

#[allow(unused_imports)]
use crate::model::{AddCollectionRequest, Collection, HistoricRequest, History, Request, Requests};
use crate::Response;

fn get_history_path() -> PathBuf {
    let mut path: PathBuf = get_pigeon_path();
    path.push("history.pigeon");
    path
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

#[cfg(not(test))]
fn build_historic_request(request: &Request, response: &Response) -> HistoricRequest {
    HistoricRequest {
        time: SystemTime::now(),
        url: request.url.clone(),
        method: request.method.clone(),
        response_status: response.status,
        size: response.size.clone(),
        speed: response.elapsed,
    }
}
#[cfg(not(test))]
pub fn add_history(request: Request, response: &Response) {
    let history_path: PathBuf = get_history_path();
    fs::write(&history_path, serde_json::to_string(&fs::read(&history_path)
        .map_or(History {
            requests: vec![build_historic_request(&request, response)]
        }, |item: Vec<u8>| {
            let mut history: History = serde_json::from_str(&*String::from_utf8(item).unwrap()).unwrap();
            history.requests.push(build_historic_request(&request, response));
            return history;
        })).unwrap()).unwrap();
}

pub fn delete_collection(collection_name: String) {
    let mut path: PathBuf = get_pigeon_path();
    path.push(&collection_name);
    println!("{:?}", path);
    fs::remove_dir_all(path).unwrap()
}

pub fn delete_request(request: Request) {
    let mut path: PathBuf = get_pigeon_path();
    if request.collection_name != "orphan" {
        path.push(request.collection_name)
    }
    let pigeon_ext: String = String::from(".pigeon");
    let mut file: String = request.name.clone();
    file.push_str(&pigeon_ext);
    path.push(file);
    fs::remove_file(&path).unwrap();
}

pub fn add_request(request: Request) {
    let mut path: PathBuf = get_pigeon_path();
    if request.collection_name != "orphan" {
        path.push(&request.collection_name);
    }
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

    write_result.is_ok()
}

pub fn get_files() -> Requests {
    let mut orphaned_requests: Vec<Request> = Vec::new();
    let mut collections: Vec<Collection> = Vec::new();

    if !get_pigeon_path().exists() {
        fs::create_dir(get_pigeon_path()).unwrap()
    }
    let res = fs::read_dir(get_pigeon_path()).unwrap();
    for result in res {
        let entry: DirEntry = result.unwrap();
        if entry.file_type().unwrap().is_file() {
            if entry.file_name().eq("history.pigeon")
                || entry.file_name().eq("scratchpad.pigeon")
                || entry.file_name().eq("environments.pigeon")
            {
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
    Requests {
        collections,
        orphaned_requests,
    }
}

fn get_collection(path: PathBuf) -> Option<Collection> {
    let traversal_result: (Vec<String>, Option<String>) = fs::read_dir(path)
        .unwrap()
        .map(|item| item.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .fold(
            (Vec::new(), None),
            |(mut requests, mut collection_config), entry| {
                if entry.file_name().eq("config.pigeon") {
                    collection_config = Some(fs::read_to_string(entry.path()).unwrap())
                } else {
                    let _ = fs::read_to_string(entry.path()).inspect(|r| requests.push(r.to_string()));
                }
                (requests, collection_config)
            },
        );

    traversal_result.1
        .map(|collection_config| deserialize_collection(collection_config, traversal_result.0))
}

fn deserialize_collection(collection_config: String, requests: Vec<String>) -> Collection {
    let mapped_requests = requests
        .iter()
        .map(|item| serde_json::from_str(item).unwrap())
        .collect();

    let config: HashMap<String, String> = serde_json::from_str(&collection_config).unwrap();

    Collection {
        name: config.get("name").unwrap().to_string(),
        description: config.get("description").unwrap().to_string(),
        requests: mapped_requests,
    }
}

fn get_pigeon_path() -> PathBuf {
    let desktop_path: String = format!("{}/Desktop", dirs::home_dir().unwrap().to_string_lossy());
    let mut path_buf = PathBuf::new();
    path_buf.push(desktop_path);
    path_buf.push("Pigeon");
    path_buf
}
#[allow(dead_code)]
fn delete_history() {
    let path = get_history_path();
    fs::write(path, "").unwrap();
}

#[cfg(test)]
#[allow(unused_variables)]
pub fn add_history(request: Request, response: &Response) {}
