use crate::assets::Tob;
use crate::data_providers::data_provider::DataProvider;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub struct MarketDataManager {
    data_provider: Arc<Mutex<dyn DataProvider + Send>>,
}

impl MarketDataManager {
    pub fn new(data_provider: Arc<Mutex<dyn DataProvider + Send>>) -> Self {
        Self { data_provider }
    }
    pub async fn listen(&self) {
        let (tx, rx) = broadcast::channel(16);

        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("WebSocket server running on ws://127.0.0.1:8080");

        let data_provider = Arc::clone(&self.data_provider);
        tokio::spawn(async move {
            let mut provider = data_provider.lock().await;
            provider.run(tx).await;
        });

        while let Ok((stream, _)) = listener.accept().await {
            let rx = rx.resubscribe();
            let handle_connection = Self::handle_connection(stream, rx);
            tokio::spawn(handle_connection);
        }
    }

    async fn handle_connection(stream: TcpStream, mut rx: broadcast::Receiver<Tob>) {
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut write, _) = ws_stream.split();

        while let Ok(tob) = rx.recv().await {
            let msg = serde_json::to_string(&tob).unwrap();
            if write.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    }
}
