// https://docs.rs/axum/latest/axum/
// https://stackoverflow.com/questions/73906747/how-to-make-an-axum-router-handle-return-different-content-type-responses
// https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e
// https://woile.dev/posts/web-app-with-template-in-rust/
// 
// https://docs.rs/axum/latest/axum/response/index.html
// 
// https://benw.is/posts/serving-static-files-with-axum#serve-files-with-axum

// https://crates.io/crates/axum
// https://github.com/joelparkerhenderson/demo-rust-axum/blob/main/src/main.rs

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(unused_attributes)]

use axum::{
  //body::{ boxed, Body, BoxBody},
  extract::{State, Path},
  routing::{get, post, get_service},
  Router,
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri}
};

use serde_json::{Value, json};
use std::sync::Arc;
use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};
//use tower::ServiceExt;
use tower_http::services::ServeDir;
//use tower_http::services::ServeFile;
use std::{io};
use serde::{Serialize, Deserialize};

use tracing_subscriber;
use tracing::{info, debug};
//use tracing_subscriber::filter::EnvFilter;

mod authapi;
use authapi::authroute;

mod testfn;
use testfn::testroute;

struct AppState {
  // ...
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[tokio::main]
async fn main(){
  // for logging to console
  // tracing_subscriber::fmt().init();
  // initialize tracing
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
  //testing logging
  //let number_of_yaks = 3;
  // this creates a new event, outside of any spans.
  //info!(number_of_yaks, "preparing to shave yaks");

  let cors = CorsLayer::new().allow_origin(Any);

  let shared_state = Arc::new(AppState { /* ... */ });

  let serve_dir = get_service(ServeDir::new("static")).handle_error(handle_error);

  // build our application with a single route
  //let app = Router::new().route("/", get(|| async { "Hello, World!" }));
  let app = Router::new()
    .with_state(shared_state)
    .nest_service("/static", serve_dir.clone())
    .route("/", get(index))
    //.route("/foo", get(|| async { "Hi from /foo" }))
    .merge(authroute()) // place here to state app error.
    .merge(testroute()) // place here to state app error.
    .fallback_service(serve_dir)
    .layer(cors);
    
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  debug!("listening on {}", addr);
  debug!("http://localhost:3000");
  //tracing::debug!("Hekll");
  //debug!("Hello Debug!");

  // run it with hyper on localhost:3000
  //axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn index() -> axum::response::Html<&'static str> {
  println!("index");
  include_str!("index.html").into()
}