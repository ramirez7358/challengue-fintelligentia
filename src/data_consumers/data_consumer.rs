use crate::assets::Tob;

pub trait DataConsumer {
    fn receive(&self, tob_receiver: Tob);
}
