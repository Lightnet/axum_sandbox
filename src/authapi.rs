
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
use tower_cookies::{Cookie, Cookies};

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

#[derive(Serialize)]
pub struct UserStatus{
  api:String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TableUser {
  id:i32,
  alias:String,
  email:String,
  passphrase:String
}

const COOKIE_NAME: &str = "token";

// https://docs.rs/sqlx/0.5.11/sqlx/macro.query_as.html
// https://github.com/launchbadge/sqlx
pub async fn signin_user(
  State(state): State<AppState>,
  cookies: Cookies,
  Json(payload): Json<UserSigin>,
) -> impl IntoResponse{
  let pool = state.pool;
  debug!("{:?}", payload);
  //println!("alias {}", payload.alias);

  /*
  //let result = sqlx::query!("SELECT * FROM users WHERE alias = $1;", payload.alias)    
  let result = sqlx::query_as::<_,TableUser>(
    "SELECT * FROM users WHERE alias = ?")
    .bind(payload.alias)   
    //.fetch(&pool)
    .fetch_one(&pool)
    //.execute(&pool)
    .await;
  //debug!("{:?}", result);
  */

  let result = sqlx::query!("SELECT * FROM users WHERE alias = $1;", payload.alias)    
    .fetch_one(&pool)
    .await;
  let mut user_status = UserStatus{
    api:"reject".into()
  };
  match result {
    Ok(r)=>{//found user
      debug!("userdata: {:?}", r);
      //debug!("userdata: {:?}", r.id);
      //debug!("userdata: {:?}", r.alias);
      //return json!({"api":"created!"}).into();

      //debug!("userdata: {:?}", r.alias.unwrap());
      // https://www.ameyalokare.com/rust/2017/10/23/rust-options.html
      //let name_string:&str= match r.alias {
        //None => "None",
        //Some(x)=> &x,
      //};

      let name_string =  r.alias.as_ref().unwrap();

      if payload.alias.eq(name_string) {
        println!("FOUND");
      }else{
        println!("NOT FOUND!");
      }

      let name_passphrase =  r.passphrase.as_ref().unwrap();
      println!("db password:{}",name_passphrase);
      println!("payload.passphrase:{}",payload.passphrase);

      if payload.passphrase.eq(name_passphrase) {
        println!("FOUND pass");
        // https://docs.rs/tower-cookies/latest/tower_cookies/cookie/struct.CookieBuilder.html#method.new
        let c = Cookie::build(COOKIE_NAME, "test")
          .path("/")
          .finish();

        //cookies.add(Cookie::new(COOKIE_NAME, ("Test").to_string()));
        cookies.add(c);

      }else{
        println!("reject pass!");
      }

      user_status.api = "passed".into();
      return (StatusCode::CREATED, Json(user_status))
    },
    Err(e)=>{//not found
      debug!("ERROR>>::{:?}", e);
      
      return (StatusCode::CREATED, Json(user_status))
    },
  }

  //let user = UserLogin {
    //alias: payload.alias,
    //passphrase: payload.passphrase,
  //};
  //Json(json!({ "data": 42 }))
  //(StatusCode::CREATED, Json(user))
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

async fn get_cookie(
  State(state): State<AppState>,
  cookies: Cookies,
) -> &'static str{
  let visited: _ = cookies
      .get(COOKIE_NAME);
      //.and_then(|c| c.value().parse().ok())
      //.unwrap_or(0);
  println!("COOKIE: {:?}",visited);


  "cookie!"
}

pub fn authroute() -> Router<AppState>{
  Router::new()
    .route("/api/signin", post(signin_user))
    .route("/api/signup", post(create_user))
    .route("/api/forgot", post(forgot_user))
    //.route("/api/echo", get(echo))
    .route("/api/ctusers", get(create_table_users))
    .route("/api/getcookie", get(get_cookie))
}