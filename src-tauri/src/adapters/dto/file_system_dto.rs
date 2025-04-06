// src/service/models/file_system.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DirectoryEntry {
    pub path: String,
    pub name: String,
    pub is_directory: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DirectoryListing {
    pub path: String,
    pub entries: Vec<DirectoryEntry>,
}
