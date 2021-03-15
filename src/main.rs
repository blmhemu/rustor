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
        .and(warp::path("download"))
        .and(warp::path::end())
        .and(warp::get())
        .and(get_file())
        .and_then(handlers::download);

    // These are web related. They can render stuff when opened in web browser.
    let web = warp::path("web");
    let web_ls = web
        .and(warp::path("ls"))
        .and(warp::path::end())
        .and(warp::get())
        .and(get_dir())
        .and_then(handlers::web_list);

    // println!("{:?}", vect.);
    // let health_route = warp::path!("health").map(|| StatusCode::OK);
    // // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = warp::path!("hello" / String / ..)
    //     .and(warp::get())
    //     .map(|name| format!("Hello, {}!", name));
    // let routes = (health_route.or(hello)).with(warp::cors().allow_any_origin());
    //
    // warp::serve(ss).run(([127, 0, 0, 1], 3030)).await;
    warp::serve(api_ls.or(api_download).or(web_ls).recover(handle_rejection))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
