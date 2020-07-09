use crate::types::*;

#[derive(Clone, Debug)]
pub struct ExecutedEvent {
    pub bid_id: OrderId,
    pub ask_id: OrderId,
    pub price: Price,
    pub qty: Qty,
    pub time: Timestamp,
}

#[derive(Clone, Debug)]
pub enum Event {
    Executed(ExecutedEvent),
}
