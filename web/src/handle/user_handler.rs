use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use crate::{Result, RestJson, AppState};

use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlRow};
use sqlx::mysql::types;
use sqlx::{Error, MySql, query, Row, Transaction};
use crate::handle::log_error;


pub async fn query_user_by_id(Extension(state): Extension<AppState>,
                              Path(id): Path<i64>) -> Result<RestJson<User>> {
    let user = sqlx::query_as::<MySql,User>(
        "SELECT id, name, age, email, create_time FROM user WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(log_error)?;

    Ok(RestJson::ok(user))
}

pub async fn query_user_by_name(Extension(state): Extension<AppState>,
                                Path(name): Path<String>) -> Result<RestJson<Vec<User>>> {
    let users = sqlx::query_as::<MySql,User>(
        "SELECT id, name, age, email, create_time FROM user WHERE name like concat('%',?,'%')")
        .bind(name)
        .fetch_all(&state.pool)
        .await
        .map_err(log_error)?;

    Ok(RestJson::ok(users))

}

pub async fn create_user(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<RestJson<String>> {
    query("insert into user(id, name, age, email, create_time) values (?,?,?,?,?)")
        .bind(payload.id)
        .bind(payload.name)
        .bind(payload.age)
        .bind(payload.email)
        .bind(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
        .execute(&state.pool)
        .await
        .map_err(log_error)?;

    Ok(RestJson::ok("".to_string()))
}

#[derive(Deserialize,Serialize,Debug,sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    age: i32,
    email: String,
    create_time: chrono::NaiveDateTime,
}
