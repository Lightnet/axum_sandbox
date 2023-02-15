// admin page only no normal account access allow here.

use axum::{
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri},
  routing::{get, post, get_service},
  Router,
};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use tracing::{info, debug};


async fn adminpage() -> axum::response::Html<&'static str> {
  println!("access");
  include_str!("access.html").into()
}

pub fn adminroute() -> Router{
  Router::new()
    //.route("/api/signin", get(signin_user))
    .route("/access", get(adminpage))
    
}
