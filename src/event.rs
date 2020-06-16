use chrono::{DateTime, Local};

pub type OrderId = i64;
pub type Price = i64;
pub type Qty = i64;
pub type Timestamp = DateTime<Local>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Side {
    Buy,
    Sell
}

fn other_side(side: Side) -> Side {
	match side {
		Side::Buy => Side::Sell,
		Side::Sell => Side::Buy
	}
}

#[derive(Clone, Debug)]
pub enum Event {
	Executed{
		bid_id: OrderId,
		ask_id: OrderId,
		price: Price,
		qty: Qty,
		time: Timestamp},
}

