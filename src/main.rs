use crate::market_simulator::MarketSimulator;

mod assets;
mod market_simulator;

#[tokio::main]
async fn main() {
    let market_simulator = MarketSimulator::new(2.0);

    market_simulator.simulate_market().await;
}
