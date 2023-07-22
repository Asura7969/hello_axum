mod services;
mod sessions;
use axum::{routing::get, Extension, Router};
use sessions::MyEvent;
use std::{net::SocketAddr, sync::Arc};

// https://juejin.cn/post/7236591682615525431?utm_source=gold_browser_extension

type TokioUnboundedSender<T> = tokio::sync::mpsc::UnboundedSender<T>;
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Use an unbounded channel to handle buffering and flushing of messages
    // to the event source...
    let (collect_tx, collect_rx) = tokio::sync::mpsc::unbounded_channel::<MyEvent>();
    let collect_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(collect_rx);

    tokio::task::spawn(async move { sessions::process(collect_rx).await });
    let collect_tx = Arc::new(collect_tx);
    let app = Router::new()
        .route("/sse", get(services::sse_handler))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(Extension(Arc::clone(&collect_tx)));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
