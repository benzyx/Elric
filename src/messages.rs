
use crate::types::*;
use serde::{Serialize, Deserialize};

/*
	Messages sent by the Clients to the servers.
*/
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
	LimitOrderMsg(LimitOrderMsg),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitOrderMsg {
	pub user: UserId,
	pub order_id: OrderId,
	pub symbol: String,
	pub qty: Qty,
	pub side: Side,
	pub price: Price,
}
