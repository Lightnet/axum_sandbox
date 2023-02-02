use axum::{
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri},
  routing::{get, post, get_service},
  Router,
};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};

//async fn root() -> &'static str {
  //"Hello, World!"
//}

async fn echo() -> &'static str {
  "Hello, World!"
}

async fn testjson() -> Json<Value>{
  Json(json!({ "data": 42 }))
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

pub fn testroute() -> Router{
  Router::new()
    .route("/foo", get(|| async { "Hi from /foo" }))
    .route("/echo", get(echo))
    .route("/json", get(testjson))
    //.route("/state", get(handler))
    //.route("/text", get(with_array_headers))
    //.route("/hello/:name", get(json_hello))
}