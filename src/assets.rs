use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Tob {
    pub bid_price: f64,
    pub bid_quantity: i32,
    pub ask_price: f64,
    pub ask_quantity: i32,
}
