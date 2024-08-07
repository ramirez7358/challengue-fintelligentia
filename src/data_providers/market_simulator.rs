use crate::assets::Tob;
use crate::data_providers::data_provider::DataProvider;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::time::Duration;
use tokio::sync::broadcast::Sender;
use tokio::time::sleep;

pub struct MarketSimulator {
    last_price: f64,
}

impl MarketSimulator {
    pub fn new(initial_price: f64) -> Self {
        MarketSimulator {
            last_price: initial_price,
        }
    }

    fn generate_tob(&mut self) -> Tob {
        let mut rng = rand::thread_rng();
        let price_fluctuation = Uniform::from(-1.0..1.0);
        let quantity_dist = Uniform::from(1..100);

        // Generate new prices based on last price with some fluctuation
        let bid_price = (self.last_price + price_fluctuation.sample(&mut rng)).max(0.0);
        let ask_price = bid_price + rng.gen_range(0.01..0.10);

        self.last_price = (bid_price + ask_price) / 2.0;

        Tob {
            bid_price,
            bid_quantity: quantity_dist.sample(&mut rng),
            ask_price,
            ask_quantity: quantity_dist.sample(&mut rng),
        }
    }
}

#[async_trait::async_trait]
impl DataProvider for MarketSimulator {
    async fn run(&mut self, sender: Sender<Tob>) {
        loop {
            let tob = self.generate_tob();
            println!("Generated TOB: {:?}", tob);
            if sender.send(tob).is_err() {
                println!("No active subscribers");
            }
            sleep(Duration::from_millis(1000)).await;
        }
    }
}
