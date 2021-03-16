mod data;
mod handlers;

use crate::data::QueryOptions;
use crate::handlers::{get_dir, get_file, get_path, handle_rejection, BASE_FOLDER};
use std::fmt::Error;
use std::path::{Path, PathBuf};
use tokio::{fs::DirEntry, io};
use warp::path::full;
use warp::{http::StatusCode, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    tokio::fs::create_dir_all(handlers::BASE_FOLDER)
        .await
        .expect("Failed to create a directory.");

    // These are API related. Only Json and Download.
    let api = warp::path("api");
    let api_ls = api
        .and(warp::path("ls"))
        .and(warp::path::end())
        .and(warp::get())
        .and(get_dir())
        .and_then(handlers::ls);
    let api_download = api
        .and(warp::path("dl"))
        .and(warp::path::end())
        .and(warp::get())
        .and(get_file())
        .and_then(handlers::download);
    let api_delete = api
        .and(warp::path("rm"))
        .and(warp::path::end())
        .and(warp::post())
        .and(get_path())
        .and_then(handlers::delete);

    // These are web related. They can render stuff when opened in web browser.
    let web = warp::path("web");
    let web_ls = web
        .and(warp::path("ls"))
        .and(warp::path::end())
        .and(warp::get())
        .and(get_dir())
        .and_then(handlers::web_list);
    let web_delete = web
        .and(warp::path("rm"))
        .and(warp::path::end())
        .and(warp::post())
        .and(get_path())
        .and_then(handlers::web_delete);

    warp::serve(
        api_ls
            .or(api_download)
            .or(api_delete)
            .or(web_ls)
            .or(web_delete)
            .recover(handle_rejection),
    )
    .run(([0, 0, 0, 0], 3030))
    .await;
}
