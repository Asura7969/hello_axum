
pub mod resp;
pub mod handle;
pub mod config;

use resp::RestJson;
use resp::*;

use axum::{
    body::Bytes,
    http::{Request, HeaderMap, StatusCode, Method, Uri},
    error_handling::{HandleError, HandleErrorLayer},
    response::{Html, Response, IntoResponse},
    routing::{get, post}, Json, Router, BoxError,
    extract::{Extension, Path}
};

use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use std::error::Error;
use serde_json::{json, Value};
use tower_http::{classify::ServerErrorsFailureClass,
                 trace::TraceLayer};

use tower::ServiceBuilder;
use tracing::{info, warn, error, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{AppState, creat_db_pool};
use crate::handle::user_handler::{create_user, query_user_by_id, query_user_by_name};


pub type Result<T> = std::result::Result<T, AppError>;


#[tokio::main]
async fn main() {

    let pool = AppState::creat_db_pool("mysql://root:123456@localhost/skeleton", 5).await;
    // let redis = AppState::creat_redis("redis://127.0.0.1/").await;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "hello_axum=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(usage))
        .route("/err/:id", get(error_handler))
        .route("/create_user", post(create_user))
        .route("/user_by_id/:id", get(query_user_by_id))
        .route("/user_by_name/:name", get(query_user_by_name))
        .layer(Extension(AppState { pool }))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &Span| {
                    info!("started {} {}", request.method(), request.uri().path())
                })
                .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                    info!("response generated in {:?}", latency)
                })
                .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
                    info!("sending {} bytes", chunk.len())
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                        warn!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        error!("something went wrong")
                    },
                ),
        );

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn error_handler(Path(id): Path<i32>) -> Result<RestJson<String>> {
    if id > 0 {
        Err(AppError{
            message:Some("".to_string()),
            cause: Some("".to_string()),
            error_type: AppErrorType::ServiceError})
    } else {
        Ok(RestJson::ok("".to_string()))
    }
}

