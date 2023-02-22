// TO DO LIST TASK
//does not work require some lib
//windows build not working required toolchain

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

pub async fn create_table_tasks(
  State(state): State<AppState>,
) -> &'static str  {
  println!("create table task");

  let pool = state.pool;

  "create tasks"
}
// GET TASKS
pub async fn get_tasks(
  State(state): State<AppState>
) -> Result<axum::Json<Vec<DataTask>>, (StatusCode, String)> {

}
//POST CREATE TASK
pub async fn post_task(
  State(state): State<AppState>,
  axum::extract::Json(task): axum::extract::Json<Task>,
)-> axum::extract::Json<Value>{

}
//UPDATE TASK
pub async fn put_task(
  State(state): State<AppState>,
  axum::extract::Json(task): axum::extract::Json<UpdateTask>,
)-> axum::extract::Json<Value>{

}
//DELETE TASK
pub async fn delete_task(
  State(state): State<AppState>,
  axum::extract::Json(task): axum::extract::Json<DeleteTask>,
)-> axum::extract::Json<Value>{

}