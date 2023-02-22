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
// https://tokio.rs/blog/2022-11-25-announcing-axum-0-6-0
use axum::{
  //body::{ boxed, Body, BoxBody},
  extract::{State, Path, FromRef},
  routing::{get, post, put, delete, get_service},
  Router,
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri}
};

use serde_json::{Value, json};
//use std::sync::Arc;
use std::{net::SocketAddr, time::Duration};

use tower_http::cors::{Any, CorsLayer};
//use tower::ServiceExt;
use tower_http::services::ServeDir;
//use tower_http::services::ServeFile;
use std::{io};
use std::env;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use tracing_subscriber;
use tracing::{info, debug};
//use tracing_subscriber::filter::EnvFilter;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Pool,Postgres};

mod authapi;
use authapi::authroute;

mod testfn;
use testfn::testroute;

//mod database;
//use database::*;

mod adminapi;
use adminapi::*;

mod sqlxtask;
use sqlxtask::*;

//mod dieseltask;
//use dieseltask::*;

// https://github.com/tokio-rs/axum/blob/main/axum-extra/src/extract/cookie/mod.rs
#[derive(Clone)]
pub struct AppState {
  //client: HttpClient,
  //database: Database,
  pool:Pool<Postgres>,
  name:String,
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[tokio::main]
async fn main(){
  dotenv().ok(); // This line loads the environment variables from the ".env" file.
  // for logging to console
  // tracing_subscriber::fmt().init();
  // initialize tracing
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_target(false)
    //.json()
    .init();
  //testing logging
  //let number_of_yaks = 3;
  // this creates a new event, outside of any spans.
  //info!(number_of_yaks, "preparing to shave yaks");

  //let dburl = std::env::var("DATABASE_URL").unwrap_or_else(|_|"None".to_string());
  //debug!("DATABASE_URL: {}", dburl);
  let db_connection_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());
  debug!("DATABASE_URL: {}", db_connection_str);

  // setup connection pool SQLX
  let pool:Pool<Postgres> = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect(&db_connection_str)
    //.connect("postgres://postgres:password@localhost/test")
    .await
    .expect("can't connect to database");

  // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
  //let row: (i64,) = sqlx::query_as("SELECT $1")
    //.bind(150_i64)
    //.fetch_one(&pool)
    //.await
    //.unwrap();

  //assert_eq!(row.0, 150);


  let cors = CorsLayer::new().allow_origin(Any);

  //let shared_state = Arc::new(AppState {
  let shared_state = AppState {
    //client: HttpClient {},
    //database: Database {},
    pool:pool,
    name:"Test".into(),
  };

  //let serve_dir = get_service(ServeDir::new("static")).handle_error(handle_error);
  let serve_dir = get_service(ServeDir::new("dist")).handle_error(handle_error);

  // build our application with a single route
  //let app = Router::new().route("/", get(|| async { "Hello, World!" }));
  let app = Router::new()
    
    //.nest_service("/static", serve_dir.clone())
    .nest_service("/dist", serve_dir.clone())
    //.route("/foo", get(|| async { "Hi from /foo" }))
    //.route("/", get(index))
    //.route("/testdb", get(testdb))
    //.route("/testdb01", get(testdb01))
    //.route("/cblog", get(create_blog))
    //.merge(todolistroute())
    .route("/api/createttask", get(create_table_tasks))
    .route("/api/task", get(get_tasks))
    .route("/api/task", post(post_task))
    .route("/api/task/:task_id", delete(delete_task))
    .route("/api/task", put(put_task))
    
    .merge(authroute()) // url > /api/name
    .merge(adminroute()) // test
    .with_state(shared_state)
    
    //.merge(testroute()) // test
    
    .fallback_service(serve_dir)
    //.fallback(handler_404)
    .layer(CookieManagerLayer::new())
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
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

//entry point index page
async fn index() -> axum::response::Html<&'static str> {
  println!("index");
  include_str!("index.html").into()
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
  tokio::signal::ctrl_c()
    .await
    .expect("expect tokio signal ctrl-c");
  println!("signal shutdown");
}