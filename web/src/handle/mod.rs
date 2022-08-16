use crate::{AppError, AppState, RestJson};

pub mod user_handler;


type HandlerResult<T> = crate::Result<RestJson<T>>;

/// 记录日志
fn log_error(err: sqlx::Error) -> sqlx::Error {
    tracing::error!("{:?}", err);
    err
}
