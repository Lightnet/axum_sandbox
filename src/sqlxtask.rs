// TO DO LIST TASK

use axum::{
  extract::{State, Path, FromRef},
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri},
  routing::{get, post, get_service},
  Router,
};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use tracing::{info, debug};

//use sqlx::{types::Json, FromRow, PgPool};

use crate::AppState;

//async fn get_tasks(
//  State(state): State<AppState>,
//) -> axum::response::Html<&'static str> {
//  println!("access");
//  include_str!("access.html").into()
//}

pub async fn create_table_tasks(
  State(state): State<AppState>,
) -> &'static str  {
  println!("create table task");

  //include_str!("access.html").into()
  let pool = state.pool;

  sqlx::query("
  drop table if exists tasks;
  ")
    .execute(&pool).await.expect("error running script");
   
  sqlx::query("
  create table tasks
  (
    id SERIAL PRIMARY KEY,
    title   varchar(100),
    author  varchar(40),
    text    text
  );
  ")
    .execute(&pool).await.expect("error running script");
  
  "create tasks"
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct DataTask{
  id:i32,
  title:String,
  author:Option<String>,
  text:String,
}
// https://users.rust-lang.org/t/need-some-help-using-axum-and-sqlx-postgres-to-send-a-json-request-payload/81291/4
pub async fn get_tasks(
  State(state): State<AppState>
) -> Result<axum::Json<Vec<DataTask>>, (StatusCode, String)> {
//) -> &'static str  {
  println!("get task");

  //include_str!("access.html").into()
  let pool = state.pool;
  //return json format
  sqlx::query_as("SELECT * FROM tasks")
    .fetch_all(&pool)
    .await
    .map(|todos| axum::Json(todos))
    .map_err(internal_error)
    
    //"get tasks"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
  title:String,
  text:String,
}

//POST CREATE TASK
pub async fn post_task(
  State(state): State<AppState>,
  //axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
  axum::extract::Json(task): axum::extract::Json<Task>,
)-> axum::extract::Json<Value>{

  let pool = state.pool;
  //debug!("Put demo JSON data: {:?}", data);
  debug!("Put demo JSON data: {:?}", task);

  //serde_json::to_string(&task).unwrap();
  //json!({"title":"test","text":"hello"}).into()
  // sqlx::query!
  let result = sqlx::query!("INSERT INTO tasks(title, text) VALUES($1, $2) RETURNING id",
    task.title.to_string(),
    task.text
  ).fetch_one(&pool)
  //execute(&pool)
    .await;
    //.last_insert_id();
  //debug!( "result: {:?}", result);
  info!( "result: {:?}", result);

  match result {
    Ok(r) => {
      info!( "result>>?: {:?}", r);
      info!( "result>>?: {:?}", r.id);
      return json!({"api":"created","id":r.id}).into();
    },
    Err(..) => {
      return json!({"api":"Something went wrong!"}).into();
    },
  }

  //json!({"title":"test","text":"hello"}).into()
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTask{
  id:i32,
  text:String,
}
//UPDATE TASK
pub async fn put_task(
  State(state): State<AppState>,
  //axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
  axum::extract::Json(task): axum::extract::Json<UpdateTask>,
)-> axum::extract::Json<Value>{

  let pool = state.pool;
  //debug!("Put demo JSON data: {:?}", data);
  debug!("Put demo JSON data: {:?}", task);


  let result = sqlx::query!(
"UPDATE tasks SET text = $2 WHERE id = $1",
task.id,
task.text
)    
    //.fetch_one(&pool)
    .execute(&pool)
    .await;
  //info!( "result: {:?}", result);

  match result {
    Ok(r) => {
      info!( "result>>?: {:?}", r);
      return json!({"api":"updated"}).into();
    },
    Err(..) => {
      return json!({"api":"Something went wrong!"}).into();
    },
  }

  //json!({"title":"test","text":"hello"}).into()
}


/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteTask{
  id:i32,
}
//DELETE TASK
pub async fn delete_task(
  State(state): State<AppState>,
  //axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
  //axum::extract::Json(task): axum::extract::Json<DeleteTask>,
  Path(task_id):Path<i32>,
)-> axum::extract::Json<Value>{

  let pool = state.pool;
  let result = sqlx::query!(
    "DELETE FROM tasks WHERE id = $1",
    task_id
    //task.id,
    )    
        //.fetch_one(&pool)
        .execute(&pool)
        .await;
      //debug!( "result: {:?}", result);
      //info!( "result: {:?}", result);


  //return json!({"api":"Something went wrong!"}).into();
  match result {
    Ok(r) => {
      info!( "result>>?: {:?}", r);
      return json!({"api":"delete"}).into();
    },
    Err(..) => {
      return json!({"api":"Something went wrong!"}).into();
    },
  }
}

//pub fn todolistroute() -> Router{
  //Router::new()
    //.route("/api/todolist", get(get_tasks))
    //.route("/api/createtasks", get(create_table_tasks))
//}