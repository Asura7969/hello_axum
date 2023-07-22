mod resp;

use std::convert::Infallible;
use std::fmt::Debug;
use axum::body::{Bytes, Full};
use axum::Json;
use axum::response::{IntoResponse, Response};
pub use resp::*;

pub enum AppErrorType {
    /// 数据库错误
    DbError,
    /// 未找到
    NotFound,
    /// 服务错误
    ServiceError,
}

pub struct AppError {
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因（上一级的错误）
    pub cause: Option<String>,
    /// 错误类型
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn new(message: Option<String>, cause: Option<String>, error_type: AppErrorType) -> Self {
        Self {message, cause, error_type}
    }

    fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }

    pub fn db_error(err: sqlx::Error) -> Self {
        // if let Some(e) = err.into_database_error() {
        //     Self {message: None, cause: Some(e.to_string()), error_type: AppErrorType::DbError}
        // } else {
        //     Self {message: None, cause: None, error_type: AppErrorType::DbError}
        // }
        match err {
            sqlx::Error::RowNotFound => Self::from_err(err, AppErrorType::NotFound),
            err => Self::from_err(err, AppErrorType::DbError)
        }

    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::db_error(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::from_err(err, AppErrorType::ServiceError)
    }
}

impl IntoResponse for AppError {

    fn into_response(self) -> Response {
        let msg = match self.message {
            Some(msg) => msg,
            None => "有错误发生".to_string(),
        };
        let res: RestJson<()> = RestJson::err(500, msg);
        Json(res).into_response()
    }
}

pub async fn usage<'a>() -> RestJson<Vec<&'a str>> {
    let data = r#"
        GET /err/:id
        GET /create_user
        GET /user_by_id/:id
        GET /user_by_name/:name
    "#;
    format!("{}", data);
    let data: Vec<&str> = data
        .split('\n')
        .into_iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    RestJson::ok(data)
}
