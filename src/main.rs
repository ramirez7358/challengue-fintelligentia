use crate::market_data_manager::MarketDataManager;
use data_providers::market_simulator::MarketSimulator;
use std::sync::Arc;
use tokio::sync::Mutex;

mod assets;
mod data_providers;
mod market_data_manager;

#[tokio::main]
async fn main() {
    let market_simulator = Arc::new(Mutex::new(MarketSimulator::new(2.0)));
    let market_data_manager = MarketDataManager::new(market_simulator);

    market_data_manager.listen().await;
}
