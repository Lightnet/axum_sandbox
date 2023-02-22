// https://infinite-improbability.org/enterprisey/
// https://github.com/shautvast/rust-rest-service
use axum::{
  //body::{ boxed, Body, BoxBody},
  extract::{State, Path, FromRef},
  routing::{get, post, get_service},
  Router,
  response::{Json, Html, IntoResponse},
  http::{StatusCode, header, Request, Response, Uri}
};

use tracing::{info, debug};

use crate::AppState;

pub async fn create_blog(// set state
  State(state): State<AppState>,
) -> &'static str  {
  println!("set app state");
  //state.name = "AA".into();
  info!("App Name: {}",state.name);
  let pool = state.pool;

  let create_database_sql = include_str!("create_database.sql");
  let statements = create_database_sql.split(";");
  for statement in statements {
    sqlx::query(statement).execute(&pool).await.expect("error running script");
  }

  info!("App Name");

  "set blog"
}
/*
drop table if exists blog_entry;
create table blog_entry
(
  title   varchar(100),
  author  varchar(40),
  text    text
);

insert into blog_entry( title, author, text)
values ('Get enterprisey with Rust', 'Sander', 'Lorem Ipsum');
insert into blog_entry( title, author, text)
values ('Get whimsical with data', 'Sander', 'Lorem Ipsum');
*/
//QUERY
/*
SELECT * FROM blog_entry
*/

pub async fn testdb(// set state
  State(state): State<AppState>,
) -> &'static str  {
  println!("set app state");
  //state.name = "AA".into();
  info!("App Name: {}",state.name);
  let pool = state.pool;

  // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
  let row: (i64,) = sqlx::query_as("SELECT $1")
    .bind(150_i64)
    .fetch_one(&pool)
    .await
    .unwrap();

  assert_eq!(row.0, 150);

  "set app state"
}

#[derive(Debug)]
struct Ticket {
	id: i64,
	name: String,
}
// https://gist.github.com/jeremychone/34d1e3daffc38eb602b1a9ab21298d10
pub async fn testdb01(// set state
  State(state): State<AppState>,
) -> &'static str  {
  println!("set app state");
  //state.name = "AA".into();
  info!("App Name: {}",state.name);
  let pool = state.pool;

  // 2) Create table if not exist yet
	let results = sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS ticket (
  id bigserial,
  name text
);"#,
	)
	.execute(&pool)
	.await;

  "set app state"
}