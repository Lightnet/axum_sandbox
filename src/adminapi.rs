// admin page only no normal account access allow here.
// https://github.com/tokio-rs/axum/pull/1543
use axum::{
  extract::{State, Path, FromRef},
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri},
  routing::{get, post, get_service},
  Router,
};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use tracing::{info, debug};

use crate::AppState;

#[derive(Deserialize,Serialize,Debug)]
pub struct AdminSigin {
  alias: String,
  passprhase: String,
}

async fn adminpage() -> axum::response::Html<&'static str> {
  println!("access");
  include_str!("access.html").into()
}

pub async fn signin_admin(
  State(state): State<AppState>,
  Json(payload): Json<AdminSigin>,
) -> impl IntoResponse{
  debug!("{:?}", payload);
  //println!("alias {}", payload.alias);

  let user = AdminSigin {
    alias: payload.alias,
    passprhase: payload.passprhase,
  };
  //Json(json!({ "data": 42 }))
  (StatusCode::CREATED, Json(user))
}

pub fn adminroute() -> Router<AppState>{
  Router::new()
    //.route("/api/signin", get(signin_user))
    .route("/access", get(adminpage))
    .route("/admin", post(signin_admin))

}
