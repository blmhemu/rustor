use crate::data;
use crate::data::{DirTemplate, FileData, QueryOptions};
use crate::handlers::reject::Reject;
use bytes::BufMut;
use futures_util::{SinkExt, StreamExt, TryFutureExt, TryStreamExt};
use mime::Mime;
use mpart_async::server::MultipartStream;
use sailfish::TemplateOnce;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::error;
use std::error::Error;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum_macros::AsRefStr;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};
use warp::http::header::{CONTENT_DISPOSITION, CONTENT_ENCODING};
use warp::http::{HeaderValue, Response, StatusCode};
use warp::hyper::Body;
use warp::multipart::{FormData, Part};
use warp::reply::Json;
use warp::{hyper, reject, Buf, Filter, Rejection, Reply, Stream};

pub(crate) const BASE_FOLDER: &str = "/Users/hbollamreddi/rustor/home";

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

pub(crate) async fn delete(path: PathBuf) -> Result<impl warp::Reply, Infallible> {
    match path.is_dir() {
        true => tokio::fs::remove_dir_all(path).await,
        false => tokio::fs::remove_file(path).await,
    };
    Ok(StatusCode::OK)
}

pub(crate) async fn web_list(path: PathBuf) -> Result<impl warp::Reply, Infallible> {
    let mut dir = tokio::fs::read_dir(path.clone()).await.unwrap();
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
        curr_path: path
            .strip_prefix(BASE_FOLDER)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        messages: file_list,
    };
    Ok(warp::reply::html(ctx.render_once().unwrap()))
}

pub(crate) async fn web_delete(path: PathBuf) -> Result<impl warp::Reply, Infallible> {
    let is_deleted = match path.is_dir() {
        true => tokio::fs::remove_dir_all(path.clone()).await,
        false => tokio::fs::remove_file(path.clone()).await,
    };
    let deletion_string = match is_deleted {
        Ok(()) => "Successfully deleted ",
        Err(_) => "Deletion failed for",
    };

    let return_dir = match &path.parent().unwrap().strip_prefix(BASE_FOLDER) {
        Ok(p) => p.to_str().unwrap(),
        Err(_) => BASE_FOLDER,
    };

    Ok(warp::reply::html(format!(
        "<html>\n<body>\n<h2>{} {}</h2>\nReturn to <a href=\"/web/ls?path={}\">parent directory</a>.\n</body>\n</html>",
        deletion_string,
        path.file_name().unwrap().to_str().unwrap(),
        urlencoding::encode(return_dir)
    )))
}

pub(crate) async fn web_create(
    path: PathBuf,
    name: String,
) -> Result<impl warp::Reply, Infallible> {
    let new_dir = path.join(name);
    let is_created = tokio::fs::create_dir(new_dir.clone()).await;
    let creation_string = match is_created {
        Ok(()) => "Successfully created ",
        Err(_) => "Creation failed for",
    };

    Ok(warp::reply::html(format!(
        "<html>\n<body>\n<h2>{} {}</h2>\nReturn to <a href=\"/web/ls?path={}\">created folder</a>.\n</body>\n</html>",
        creation_string,
        new_dir.strip_prefix(BASE_FOLDER).unwrap().to_str().unwrap(),
        urlencoding::encode(new_dir.strip_prefix(BASE_FOLDER).unwrap().to_str().unwrap()),
    )))
}

pub(crate) async fn web_upload(
    path: PathBuf,
    mime: Mime,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin,
) -> Result<impl warp::Reply, Rejection> {
    let mut success_set: HashSet<String> = HashSet::new();
    let boundary = mime.get_param("boundary").map(|v| v.to_string()).unwrap();

    let mut stream = MultipartStream::new(
        boundary,
        body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining())),
    );

    while let Ok(Some(mut field)) = stream.try_next().await {
        println!("Field received:{}", field.name().unwrap());
        if let Ok(filename) = field.filename() {
            println!("Field filename:{}", filename);
            let sanitize_filename = sanitize_filename::sanitize(&filename);
            if sanitize_filename.trim().is_empty() || sanitize_filename != filename {
                return Err(warp::reject::custom(CustomErrors::InvalidName));
            };
            let file = path.join(&sanitize_filename);
            if file.clone().exists() {
                return Err(warp::reject::custom(CustomErrors::FileAlreadyExists));
            };
            let mut file = tokio::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(file)
                .await
                .unwrap();
            while let Ok(Some(bytes)) = field.try_next().await {
                file.write_all(&bytes).await;
            }
            success_set.insert(String::from(sanitize_filename));
        }
    }

    Ok(warp::reply::html(format!(
            "<html>\n<body>\n<h2>{} {}</h2>\nReturn to <a href=\"/web/ls?path={}\">uploaded folder</a>.\n</body>\n</html>",
            "Successfully uploaded the following files \n",
            success_set.iter().fold(String::new(), |mut acc: String, file| {
                acc.push_str(file);
                acc.push_str(" | ");
                return acc;
            }),
            urlencoding::encode(path.strip_prefix(BASE_FOLDER).unwrap().to_str().unwrap()),
        )))
}

// Make error handling even better
#[derive(Debug, AsRefStr)]
pub(crate) enum CustomErrors {
    NotAFileError,
    NotADirError,
    InvalidName,
    FileAlreadyExists,
    InvalidPathError,
    TokioError(io::Error),
}

impl reject::Reject for CustomErrors {}

// #[derive(Debug)]
// pub struct NotAFileError;
// #[derive(Debug)]
// pub struct NotADirError;
// #[derive(Debug)]
// pub struct InvalidName;
// #[derive(Debug)]
// pub struct FileAlreadyExists;
// #[derive(Debug)]
// pub struct InvalidPathError;
// #[derive(Debug)]
// pub struct TokioError(io::Error);
//
// impl reject::Reject for NotAFileError {}
// impl reject::Reject for NotADirError {}
// impl reject::Reject for InvalidName {}
// impl reject::Reject for FileAlreadyExists {}
// impl reject::Reject for InvalidPathError {}
// impl reject::Reject for TokioError {}

pub(crate) fn reject_invalid_names() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::query::<QueryOptions>().and_then(|opts: QueryOptions| async {
        match opts.dirname {
            Some(name) => {
                let sanitize_filename = sanitize_filename::sanitize(&name);
                if sanitize_filename.trim().is_empty() || sanitize_filename != name {
                    return Err(warp::reject::custom(CustomErrors::InvalidName));
                };
                Ok(sanitize_filename)
            }
            None => Err(warp::reject::custom(CustomErrors::InvalidName)),
        }
    })
}

// TODO: Merge this into above
pub(crate) fn get_newdir_name() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::query::<QueryOptions>().and_then(|opts: QueryOptions| async {
        match opts.dirname {
            Some(name) => {
                let sanitize_filename = sanitize_filename::sanitize(&name);
                if sanitize_filename.trim().is_empty() || sanitize_filename != name {
                    return Err(warp::reject::custom(CustomErrors::InvalidName));
                };
                Ok(sanitize_filename)
            }
            None => Err(warp::reject::custom(CustomErrors::InvalidName)),
        }
    })
}

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
        Err(e) => Err(warp::reject::custom(CustomErrors::TokioError(e))),
        // Todo: Better handling.
        Ok(path) if !path.starts_with(BASE_FOLDER) => {
            Err(warp::reject::custom(CustomErrors::InvalidPathError))
        }
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
        false => Err(warp::reject::custom(CustomErrors::NotAFileError)),
    }
}

async fn get_valid_dir(path: PathBuf) -> Result<PathBuf, Rejection> {
    match tokio::fs::metadata(path.clone())
        .await
        .map(|m| m.is_dir())
        .unwrap_or(false)
    {
        true => Ok(path),
        false => Err(warp::reject::custom(CustomErrors::NotADirError)),
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
    } else if let Some(CustomErrors::NotAFileError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "NOT_A_FILE";
        description = Some("This operation cannot be done on a directory.");
    } else if let Some(CustomErrors::NotADirError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "NOT_A_DIRECTORY";
        description = Some("This operation cannot be done on a file.");
    } else if let Some(CustomErrors::InvalidName) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "INVALID_NAME";
        description = Some("Please provide a proper name.");
    } else if let Some(CustomErrors::FileAlreadyExists) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "FILE_ALREADY_EXISTS";
        description = Some("Please provide a different name or delete existing file.");
    } else if let Some(CustomErrors::InvalidPathError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "INVALID_PATH";
        description = Some("Please provide a valid path.");
    } else if let Some(CustomErrors::TokioError(_)) = err.find() {
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
