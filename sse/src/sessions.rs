use crate::TokioUnboundedSender;
#[derive(Default, Debug, Clone)]
pub(crate) struct Signal {
    pub(crate) event: String,
    pub(crate) data: String,
}

impl Signal {
    pub(crate) fn new(event: String, data: String) -> Self {
        Signal { event, data }
    }
}
pub(super) enum MyEvent {
    SSE(TokioUnboundedSender<Signal>),
}

pub(super) async fn process(
    mut collect_rx: tokio_stream::wrappers::UnboundedReceiverStream<MyEvent>,
) {
    use tokio_stream::StreamExt as _;

    while let Some(sessions) = collect_rx.next().await {
        match sessions {
            MyEvent::SSE(sender) => {
                for i in 0..5 {
                    let _ = sender.send(Signal::new("send".to_string(), i.to_string()));
                }
            }
        }
    }
}
