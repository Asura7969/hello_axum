use crate::{sessions::MyEvent, TokioUnboundedSender};
use axum::{
    response::{sse::Event, Sse},
    Extension,
};
use std::{convert::Infallible, sync::Arc, time::Duration};

pub(super) async fn sse_handler(
    Extension(collect_tx): Extension<Arc<TokioUnboundedSender<MyEvent>>>,
) -> Sse<impl futures::stream::Stream<Item = Result<Event, Infallible>>> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    use tokio_stream::StreamExt as _;
    let _ = collect_tx.send(MyEvent::SSE(tx));

    let stream = async_stream::stream! {
        loop {
            let signal = rx.recv().await;
            yield signal
        };
    }
    .map(|signal| {
        let event = if let Some(signal) = signal {
            Event::default()
                .event(signal.event.clone())
                .data(signal.data)
        } else {
            Event::default().data(format!("None"))
        };
        println!("发送event: {:?}", event);
        event
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));
    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15)))
}
