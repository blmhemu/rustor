use crate::data;
use crate::data::{DirTemplate, FileData, QueryOptions};
use sailfish::TemplateOnce;
use serde_json::to_string;
use std::convert::Infallible;
use std::error;
use std::error::Error;
use std::path::{Path, PathBuf};
use tokio::io;
use tokio_util::codec::{BytesCodec, FramedRead};
use tokio_util::io::StreamReader;
use warp::http::header::{CONTENT_DISPOSITION, CONTENT_ENCODING};
use warp::http::{HeaderValue, Response, StatusCode};
use warp::hyper::Body;
use warp::reply::Json;
use warp::{hyper, reject, Filter, Rejection, Reply};

pub(crate) const BASE_FOLDER: &str = "/Users/hbollamreddi/rustor";

pub(crate) async fn web_list(path: PathBuf) -> Result<impl warp::Reply, Infallible> {
    let mut dir = tokio::fs::read_dir(path).await.unwrap();
    let mut file_list = Vec::new();
    while let Some(child) = dir.next_entry().await.unwrap() {
        file_list.push(FileData {
            path: urlencoding::encode(
                child
                    .path()
                    .strip_prefix(BASE_FOLDER)
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ),
            is_dir: child.path().is_dir(),
            name: child.file_name().to_str().unwrap().to_string(),
        });
    }

    let ctx = DirTemplate {
        messages: file_list,
    };
    Ok(warp::reply::html(ctx.render_once().unwrap()))
}

pub(crate) async fn ls(path: PathBuf) -> Result<impl warp::Reply, Infallible> {
    let mut dir = tokio::fs::read_dir(path).await.unwrap();
    let mut file_list = Vec::new();
    while let Some(child) = dir.next_entry().await.unwrap() {
        file_list.push(FileData {
            path: child
                .path()
                .strip_prefix(BASE_FOLDER)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            is_dir: child.path().is_dir(),
            name: child.file_name().to_str().unwrap().to_string(),
        });
    }

    Ok(warp::reply::json(&file_list))
}

pub(crate) async fn download(path: PathBuf) -> Result<impl warp::Reply, Infallible> {
    let file = tokio::fs::File::open(path.clone()).await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    let disposition = format!(
        "attachment; filename = \"{}\"",
        path.file_name().unwrap().to_str().unwrap().to_string()
    );
    Ok(Response::builder()
        .header(CONTENT_DISPOSITION, disposition)
        .body(body))
}

#[derive(Debug)]
pub struct NotAFileError;
#[derive(Debug)]
pub struct NotADirError;
#[derive(Debug)]
pub struct InvalidPathError;
#[derive(Debug)]
pub struct TokioError(io::Error);

impl reject::Reject for NotAFileError {}
impl reject::Reject for NotADirError {}
impl reject::Reject for InvalidPathError {}
impl reject::Reject for TokioError {}

pub(crate) fn get_dir() -> impl Filter<Extract = (PathBuf,), Error = Rejection> + Copy {
    get_path().and_then(get_valid_dir)
}

pub(crate) fn get_file() -> impl Filter<Extract = (PathBuf,), Error = Rejection> + Copy {
    get_path().and_then(get_valid_file)
}

pub(crate) fn get_path() -> impl Filter<Extract = (PathBuf,), Error = Rejection> + Copy {
    warp::query::<QueryOptions>().and_then(get_canonical_path)
}

/// Gets a sanitized canonical path which is safe from path traversal attack.
async fn get_canonical_path(opts: QueryOptions) -> Result<PathBuf, Rejection> {
    // We expect relative path. If absolute path is queried, we reject the request.
    let rel_path = opts.path.unwrap_or(PathBuf::default());
    let mut abs_path = PathBuf::from(BASE_FOLDER).join(rel_path);
    // Canonicalization also tests for existence.
    let abs_path = tokio::fs::canonicalize(abs_path).await;
    match abs_path {
        Err(e) => Err(warp::reject::custom(TokioError(e))),
        // Todo: Better handling.
        Ok(path) if !path.starts_with(BASE_FOLDER) => Err(warp::reject::custom(InvalidPathError)),
        Ok(path) => Ok(path),
    }
}

async fn get_valid_file(path: PathBuf) -> Result<PathBuf, Rejection> {
    match tokio::fs::metadata(path.clone())
        .await
        .map(|m| m.is_file())
        .unwrap_or(false)
    {
        true => Ok(path),
        false => Err(warp::reject::custom(NotAFileError)),
    }
}

async fn get_valid_dir(path: PathBuf) -> Result<PathBuf, Rejection> {
    match tokio::fs::metadata(path.clone())
        .await
        .map(|m| m.is_dir())
        .unwrap_or(false)
    {
        true => Ok(path),
        false => Err(warp::reject::custom(NotADirError)),
    }
}

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    let description;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
        description = None;
    } else if let Some(NotAFileError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "NOT_A_FILE";
        description = Some("This operation cannot be done on a directory.");
    } else if let Some(NotADirError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "NOT_A_DIRECTORY";
        description = Some("This operation cannot be done on a file.");
    } else if let Some(InvalidPathError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "INVALID_PATH";
        description = Some("Please provide a valid path.");
    } else if let Some(TokioError(_)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "INVALID_PATH";
        description = Some("Please provide a valid path.");
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
        description = None;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
        description = None;
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
        description = None;
    }

    let json = warp::reply::json(&data::ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
        description: description.map(Into::into),
    });

    Ok(warp::reply::with_status(json, code))
}
