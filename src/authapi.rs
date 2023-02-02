
use axum::{
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri},
  routing::{get, post, get_service},
  Router,
};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct UserLogin {
  alias: String,
  passprhase: String,
}

#[derive(Deserialize)]
pub struct UserSigin {
  alias: String,
  passprhase: String,
}

pub async fn signin_user(
  Json(payload): Json<UserSigin>,
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
pub struct CreateUser {
  alias: String,
  passphrase: String,
}

#[derive(Serialize)]
pub struct User {
  id: u64,
  alias: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
  //println!("alias {}", payload.alias);

  let user = User {
    id: 1337,
    alias: payload.alias,
  };
  //Json(json!({ "data": 42 }))
  (StatusCode::CREATED, Json(user))
}

async fn echo() -> &'static str {
  "Hello, World!"
}

pub fn authroute() -> Router{
  Router::new()
    .route("/api/signin", post(signin_user))
    .route("/api/signup", post(create_user))
    .route("/api/echo", get(echo))
}