// https://docs.rs/axum/latest/axum/
// https://stackoverflow.com/questions/73906747/how-to-make-an-axum-router-handle-return-different-content-type-responses
// https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e
// https://woile.dev/posts/web-app-with-template-in-rust/
// 
// https://docs.rs/axum/latest/axum/response/index.html
// 
// https://benw.is/posts/serving-static-files-with-axum#serve-files-with-axum

// https://crates.io/crates/axum







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
use tower_http::cors::{Any, CorsLayer};
//use tower::ServiceExt;
use tower_http::services::ServeDir;
//use tower_http::services::ServeFile;
use std::{io};
use serde::{Serialize, Deserialize};



struct AppState {
  // ...
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[tokio::main]
async fn main(){

  tracing_subscriber::fmt::init(); 

  let cors = CorsLayer::new().allow_origin(Any);

  let shared_state = Arc::new(AppState { /* ... */ });

  println!("http://localhost:3000");

  let serve_dir = get_service(ServeDir::new("static")).handle_error(handle_error);


  // build our application with a single route
  //let app = Router::new().route("/", get(|| async { "Hello, World!" }));
  let app = Router::new()
    .route("/foo", get(|| async { "Hi from /foo" }))
    //.nest_service("/static", get(file_handler))
    .nest_service("/static", serve_dir.clone())
    //.route("/", get(root))
    .route("/", get(index))
    .route("/echo", get(echo))
    .route("/json", get(testjson))
    .route("/state", get(handler))
    .route("/text", get(with_array_headers))
    .route("/hello/:name", get(json_hello))

    .route("/api/signin", post(create_user))
    //.route("/api/signup", post(post_SignUp))
    //.route("/api/forgot", get(json_hello))
    //.route("/api/token", get(json_hello))
    /*
    .route("/static", get_service(ServeFile::new("static/index.html"))
    .handle_error(|error: io::Error| async move { 
      ( 
        StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Unhandled internal error: {}", error),
      )
    }))
    */
    .with_state(shared_state)
    .fallback_service(serve_dir)
    .layer(cors);

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn index() -> &'static str {
  "Hello, World!"
}


#[derive(Serialize)]
struct UserLogin {
  alias: String,
  passprhase: String,
}

async fn post_SignIn(
  Json(payload): Json<UserLogin>,
) -> impl IntoResponse{
  println!("alias {}", payload.alias);

  let user = UserLogin {
    alias: payload.alias,
    passprhase: payload.passprhase,
  };
  //Json(json!({ "data": 42 }))
  (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
  alias: String,
  passphrase: String,
}

#[derive(Serialize)]
struct User {
  id: u64,
  alias: String,
}

async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
  //println!("alias {}", payload.alias);

  let user = User {
    id: 1337,
    alias: payload.alias,
  };
  //Json(json!({ "data": 42 }))
  (StatusCode::CREATED, Json(user))
}





async fn root() -> &'static str {
  "Hello, World!"
}


async fn echo() -> &'static str {
  "Hello, World!"
}

async fn testjson() -> Json<Value>{
  Json(json!({ "data": 42 }))
}

async fn handler(
  State(state): State<Arc<AppState>>,
)  -> &'static str{
  // ...
  println!("state!");

  "state"
}

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
  Html("<h1>Hello, World!</h1>")
}

async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
  let greeting = name.as_str();
  let hello = String::from("Hello ");

  (StatusCode::OK, Json(json!({"message": hello + greeting })))
}

// Or an array of tuples to more easily build the headers
async fn with_array_headers() -> impl IntoResponse {
  ([(header::CONTENT_TYPE, "text/plain")], "foo")
}


/*
pub async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
  let res = get_static_file(uri.clone()).await?;
  println!("{:?}", res);

  if res.status() == StatusCode::NOT_FOUND {
      // try with `.html`
      // TODO: handle if the Uri has query parameters
      match format!("{}.html", uri).parse() {
          Ok(uri_html) => get_static_file(uri_html).await,
          Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
      }
  } else {
      Ok(res)
  }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
  let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

  // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
  // When run normally, the root is the workspace root
  match ServeDir::new("../../wasm/pkg").oneshot(req).await {
      Ok(res) => Ok(res.map(boxed)),
      Err(err) => Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Something went wrong: {}", err),
      )),
  }
}
*/
