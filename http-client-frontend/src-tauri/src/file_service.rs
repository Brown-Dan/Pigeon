use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Collection {
    name: String,
    description: String,
    requests: Vec<Request>,
}

#[derive(Serialize, Deserialize)]
struct Request {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Requests {
    collections: Vec<Collection>,
    orphaned_requests: Vec<Request>,
}

pub fn get_files() -> Requests {
    let mut orphaned_requests: Vec<Request> = Vec::new();
    let mut collections: Vec<Collection> = Vec::new();

    let desktop_path: String = format!("{}/Desktop", dirs::home_dir().unwrap().to_string_lossy());
    let mut pigeon_folder = PathBuf::new();
    pigeon_folder.push(desktop_path);
    pigeon_folder.push("Pigeon");

    let res = fs::read_dir(pigeon_folder).unwrap();
    for result in res {
        let entry: DirEntry = result.unwrap();
        if entry.file_type().unwrap().is_file() {
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
        name: config.get("collectionName").unwrap().to_string(),
        description: config.get("description").unwrap().to_string(),
        requests: mapped_requests,
    };
}
