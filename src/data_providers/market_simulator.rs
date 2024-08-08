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

        let fluctuation = price_fluctuation.sample(&mut rng);
        let new_price = (self.last_price + fluctuation).max(0.0);

        let bid_price = (new_price - rng.gen_range(0.01..0.05)).max(0.0);
        let ask_price = new_price + rng.gen_range(0.01..0.05);

        self.last_price = new_price;

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
            sleep(Duration::from_millis(500)).await;
        }
    }
}
