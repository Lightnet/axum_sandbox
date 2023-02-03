use axum::{
  response::{Json, Html, IntoResponse},
  extract::{State, Path, FromRef},
  http::{StatusCode, header, Request, Response, Uri},
  routing::{get, post, get_service},
  Router,
};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use tracing::{info, debug};

use crate::AppState;


//async fn root() -> &'static str {
  //"Hello, World!"
//}

async fn echo() -> &'static str {
  "Hello, World!"
}

async fn testjson() -> Json<Value>{
  Json(json!({ "data": 42 }))
}

async fn gstate(// get state
  //State(client): State<HttpClient>,
  //State(database): State<Database>,
  State(state): State<AppState>,
) -> &'static str  {
  println!("get app state");
  info!("App Name: {}",state.name);

  "get app state"
}

pub async fn sstate(// set state
  State(state): State<AppState>,
) -> &'static str  {
  println!("set app state");
  //state.name = "AA".into();
  info!("App Name: {}",state.name);

  "set app state"
}

//async fn handler(
  //State(state): State<Arc<AppState>>,
//)  -> &'static str{
  // ...
  //println!("state!");
  //"state"
//}

// `Html` gives a content-type of `text/html`
//async fn html() -> Html<&'static str> {
  //Html("<h1>Hello, World!</h1>")
//}

//async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
  //let greeting = name.as_str();
  //let hello = String::from("Hello ");
  //(StatusCode::OK, Json(json!({"message": hello + greeting })))
//}

// Or an array of tuples to more easily build the headers
//async fn with_array_headers() -> impl IntoResponse {
  //([(header::CONTENT_TYPE, "text/plain")], "foo")
//}


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

const COOKIE_NAME: &str = "visited";

async fn cookiecounter(cookies: Cookies) -> String {
  let visited = cookies
      .get(COOKIE_NAME)
      .and_then(|c| c.value().parse().ok())
      .unwrap_or(0);
  if visited > 10 {
      cookies.remove(Cookie::new(COOKIE_NAME, ""));
      "Counter has been reset".into()
  } else {
      cookies.add(Cookie::new(COOKIE_NAME, (visited + 1).to_string()));
      format!("You've been here {} times before", visited)
  }
}

async fn set_cookie() -> Json<Value>{
  Json(json!({ "data": 42 }))
}
async fn get_cookie() -> Json<Value>{
  Json(json!({ "data": 42 }))
}

pub fn testroute() -> Router{
  Router::new()
    //.route("/gstate", get(gstate))
    //.route("/sstate", get(sstate))
    .route("/foo", get(|| async { "Hi from /foo" }))
    .route("/echo", get(echo))
    .route("/json", get(testjson))
    .route("/scookie", get(set_cookie))
    .route("/gcookie", get(get_cookie))
    .route("/ccookie", get(cookiecounter))
    //.route("/state", get(handler))
    //.route("/text", get(with_array_headers))
    //.route("/hello/:name", get(json_hello))
}