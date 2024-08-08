use crate::data_consumers::data_consumer::DataConsumer;
use crate::data_consumers::futures_contract_consumer::FuturesContractConsumer;
use crate::data_consumers::ws_consumer::WsConsumer;
use crate::market_data_manager::MarketDataManager;
use data_providers::market_simulator::MarketSimulator;
use std::sync::Arc;
use tokio::sync::Mutex;

mod assets;
mod data_consumers;
mod data_providers;
mod market_data_manager;

#[tokio::main]
async fn main() {
    let market_simulator = Arc::new(Mutex::new(MarketSimulator::new(2.0)));
    let data_consumers: Vec<Box<dyn DataConsumer + Send + Sync>> =
        vec![WsConsumer::new().await, FuturesContractConsumer::new()];
    let market_data_manager = MarketDataManager::new(market_simulator, data_consumers);

    market_data_manager.listen().await;
}
