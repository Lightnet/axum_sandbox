
use axum::{
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri},
  routing::{get, post, get_service},
  Router,
};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use tracing::{info, debug};

#[derive(Serialize)]
pub struct UserLogin {
  alias: String,
  passprhase: String,
}

#[derive(Deserialize,Debug)]
pub struct UserSigin {
  alias: String,
  passprhase: String,
}

pub async fn signin_user(
  Json(payload): Json<UserSigin>,
) -> impl IntoResponse{
  debug!("{:?}", payload);
  //println!("alias {}", payload.alias);

  let user = UserLogin {
    alias: payload.alias,
    passprhase: payload.passprhase,
  };
  //Json(json!({ "data": 42 }))
  (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize, Debug)]
pub struct CreateUser {
  alias: String,
  passphrase: String,
}

#[derive(Serialize)]
pub struct User {
  id: u64,
  alias: String,
}

// https://docs.rs/tracing-stackdriver/latest/tracing_stackdriver/
// https://docs.rs/axum/0.2.6/axum/index.html
pub async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
  //println!("alias {}", payload.alias);

  //let deserialized = serde_json::from_str(payload).unwrap();
  //println!("{:?}", deserialized);
  //if payload {
    // We got a valid JSON payload
    //println!("{:?}", payload);
  //} else {
    // Payload wasn't valid JSON
  //}

  debug!("{:?}", payload);



  //info!("json {}", payload);
  let user = User {
    id: 1337,
    alias: payload.alias,
  };
  //Json(json!({ "data": 42 }))
  (StatusCode::CREATED, Json(user))
}


#[derive(Deserialize, Debug)]
pub struct RecoveryUser {
  alias: String,
  email: String,
}

pub async fn forgot_user(Json(payload): Json<RecoveryUser>,) -> impl IntoResponse {
  //println!("alias {}", payload.alias);
  debug!("{:?}", payload);

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
    .route("/api/forgot", post(forgot_user))
    .route("/api/echo", get(echo))
}