use crate::assets::Tob;
use tokio::sync::broadcast;

#[async_trait::async_trait]
pub trait DataProvider {
    async fn run(&mut self, sender: broadcast::Sender<Tob>);
}
