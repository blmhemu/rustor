use crate::handlers::BASE_FOLDER;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(TemplateOnce)] // automatically implement `TemplateOnce` trait
#[template(path = "index.stpl")] // specify the path to template
pub(crate) struct DirTemplate {
    // data to be passed to the template
    pub curr_path: String,
    pub messages: Vec<FileData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FileData {
    pub path: String,
    pub is_dir: bool,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryOptions {
    pub path: Option<PathBuf>,
}

/// An API error serializable to JSON.
#[derive(Serialize)]
pub(crate) struct ErrorMessage {
    pub code: u16,
    pub message: String,
    pub description: Option<String>,
}
