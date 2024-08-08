use crate::assets::Tob;
use crate::data_consumers::data_consumer::DataConsumer;
use rand::Rng;
use std::collections::HashMap;
use std::time::Duration;

pub struct FuturesContractConsumer {
    contracts: HashMap<String, f64>, // Map of contract name to interest rate
    contract_expires: HashMap<String, Duration>, // Map of contract name to expiry duration
}

impl FuturesContractConsumer {
    pub fn new() -> Box<dyn DataConsumer + Send + Sync> {
        let mut contracts = HashMap::new();
        let mut contract_expires = HashMap::new();

        contracts.insert("Contract1".to_string(), 0.02); // 2% annual risk-free rate
        contracts.insert("Contract2".to_string(), 0.025); // 2.5% annual risk-free rate
        contracts.insert("Contract3".to_string(), 0.03); // 3% annual risk-free rate

        contract_expires.insert(
            "Contract1".to_string(),
            Duration::from_secs(30 * 24 * 60 * 60),
        ); // 30 days
        contract_expires.insert(
            "Contract2".to_string(),
            Duration::from_secs(60 * 24 * 60 * 60),
        ); // 60 days
        contract_expires.insert(
            "Contract3".to_string(),
            Duration::from_secs(90 * 24 * 60 * 60),
        ); // 90 days

        Box::new(Self {
            contracts,
            contract_expires,
        })
    }

    /*
    F=S×(1+r×T)

    Donde:

        F es el precio del futuro.
        S es el precio actual del activo subyacente (en este caso, el precio de demanda o de oferta del TOB).
        r es la tasa libre de riesgo.
        T es el tiempo hasta el vencimiento del contrato.
    */
    fn calculate_and_quote(&self, tob: &Tob) {
        for (contract, rate) in &self.contracts {
            if let Some(expiry) = self.contract_expires.get(contract) {
                let future_price = tob.ask_price
                    * (1.0 + rate * (expiry.as_secs_f64() / (365.0 * 24.0 * 60.0 * 60.0)));
                let mut rng = rand::thread_rng();
                let quantity: u32 = rng.gen_range(1..100);

                println!(
                    "Contract: {}, Expiry: {:?}, Future Price: {:.2}, Quantity: {}",
                    contract, expiry, future_price, quantity
                );
            }
        }
    }
}
impl DataConsumer for FuturesContractConsumer {
    fn receive(&self, tob_receiver: Tob) {
        self.calculate_and_quote(&tob_receiver);
    }
}
