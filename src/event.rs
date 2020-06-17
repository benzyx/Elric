use crate::types::*;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[allow(dead_code)]
impl Side {
    fn other(self) -> Side {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

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
