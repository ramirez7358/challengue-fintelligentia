use crate::data_consumers::data_consumer::DataConsumer;
use crate::data_providers::data_provider::DataProvider;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub struct MarketDataManager {
    data_provider: Arc<Mutex<dyn DataProvider + Send>>,
    data_consumers: Vec<Box<dyn DataConsumer + Send + Sync>>,
}

impl MarketDataManager {
    pub fn new(
        data_provider: Arc<Mutex<dyn DataProvider + Send>>,
        data_consumers: Vec<Box<dyn DataConsumer + Send + Sync>>,
    ) -> Self {
        Self {
            data_provider,
            data_consumers,
        }
    }
    pub async fn listen(&self) {
        let (tob_sender, mut tob_receiver) = broadcast::channel(16);

        let data_provider = Arc::clone(&self.data_provider);

        tokio::spawn(async move {
            let mut provider = data_provider.lock().await;
            provider.run(tob_sender).await;
        });

        loop {
            match tob_receiver.recv().await {
                Ok(tob) => self.data_consumers.iter().for_each(|dc| {
                    dc.receive(tob.clone());
                }),
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
    }
}
