
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

pub async fn create_table_users(
  State(state): State<AppState>,
) -> &'static str  {
  println!("create table users");

  //include_str!("access.html").into()
  let pool = state.pool;

  sqlx::query("drop table if exists users;")
    .execute(&pool).await.expect("error running script");
  
  sqlx::query("
  CREATE TABLE users
  (
    id SERIAL PRIMARY KEY,
    alias  varchar(100),
    email  varchar(40),
    passphrase varchar(100)
  );
  ")
    .execute(&pool).await.expect("error running script");
  
  
  "create table users"
}


#[derive(Serialize)]
pub struct UserLogin {
  alias: String,
  passphrase: String,
}

#[derive(Deserialize,Debug)]
pub struct UserSigin {
  alias: String,
  passphrase: String,
}

pub async fn signin_user(
  Json(payload): Json<UserSigin>,
) -> impl IntoResponse{
  debug!("{:?}", payload);
  //println!("alias {}", payload.alias);

  let user = UserLogin {
    alias: payload.alias,
    passphrase: payload.passphrase,
  };
  //Json(json!({ "data": 42 }))
  (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize, Debug)]
pub struct CreateUser {
  alias: String,
  passphrase: String,
  email:String,
}

#[derive(Serialize)]
pub struct User {
  id: u64,
  alias: String,
}

#[derive(Serialize)]
pub struct RegisterStatus{
  api:String,
}

// https://docs.rs/tracing-stackdriver/latest/tracing_stackdriver/
// https://docs.rs/axum/0.2.6/axum/index.html
pub async fn create_user(
  State(state): State<AppState>,
  Json(payload): Json<CreateUser>,
) -> impl IntoResponse {

  let pool = state.pool;
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
  
  let result = sqlx::query!("SELECT * FROM users WHERE alias = $1;", payload.alias)    
        .fetch_one(&pool)
        //.execute(&pool)
        .await;
  //debug!("{:?}", result);
  let mut register_status = RegisterStatus{
    api:"exist".into()
  };

  match result {
    Ok(r)=>{
      debug!("{:?}", r);
      //return json!({"api":"created!"}).into();

      return (StatusCode::CREATED, Json(register_status))
    },
    Err(e)=>{
      debug!("{:?}", e);
      
      let ralias = sqlx::query!("INSERT INTO users(alias, email, passphrase) VALUES($1, $2, $3) RETURNING id",
        payload.alias,
        payload.email,
        payload.passphrase
      )
      .fetch_one(&pool)
      .await;
      debug!("{:?}>>>", ralias);

      match ralias {
        Ok(ra)=>{//created
          register_status.api = "created".into();
          return (StatusCode::CREATED, Json(register_status))
        },
        Err(ea)=>{//error fail
          return (StatusCode::CREATED, Json(register_status))
        }
      }
      //return (StatusCode::CREATED, Json(register_status))
    },
  }


  //info!("json {}", payload);
  //let user = User {
    //id: 1337,
    //alias: payload.alias,
  //};
  //Json(json!({ "data": 42 }))
  //(StatusCode::CREATED, Json(user))
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

pub fn authroute() -> Router<AppState>{
  Router::new()
    .route("/api/signin", post(signin_user))
    .route("/api/signup", post(create_user))
    .route("/api/forgot", post(forgot_user))
    //.route("/api/echo", get(echo))
    .route("/api/ctusers", get(create_table_users))
}