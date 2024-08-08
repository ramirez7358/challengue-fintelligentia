use crate::assets::Tob;
use crate::data_consumers::data_consumer::DataConsumer;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub struct WsConsumer {
    internal_sender: Sender<Tob>,
    internal_receiver: Receiver<Tob>,
}

impl WsConsumer {
    pub async fn new() -> Box<dyn DataConsumer + Send + Sync> {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("WebSocket server running on ws://127.0.0.1:8080");

        let (tx, rx) = broadcast::channel(10);

        let ws_consumer = Arc::new(Self {
            internal_sender: tx.clone(),
            internal_receiver: rx.resubscribe(),
        });

        let ws_consumer_clone = Arc::clone(&ws_consumer);

        tokio::spawn(async move {
            ws_consumer_clone.accept_connections(listener).await;
        });

        Box::new(Self {
            internal_sender: tx,
            internal_receiver: rx,
        })
    }

    async fn accept_connections(self: Arc<Self>, listener: TcpListener) {
        while let Ok((stream, _)) = listener.accept().await {
            let rx = self.internal_receiver.resubscribe();
            let handle_connection = Self::handle_connection(stream, rx);
            tokio::spawn(handle_connection);
        }
    }

    async fn handle_connection(stream: TcpStream, mut internal_receiver: Receiver<Tob>) {
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut write, _) = ws_stream.split();

        while let Ok(tob) = internal_receiver.recv().await {
            let msg = serde_json::to_string(&tob).unwrap();
            if write.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    }
}

impl DataConsumer for WsConsumer {
    fn receive(&self, tob_receiver: Tob) {
        self.internal_sender
            .send(tob_receiver)
            .expect("TODO: panic message");
    }
}
