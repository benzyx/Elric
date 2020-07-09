use chrono::{DateTime, Local};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


pub type Symbol = String;
pub type OrderId = i64;
pub type Price = i64;
pub type Qty = i64;
pub type Timestamp = DateTime<Local>;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

pub type UserId = i64;

#[allow(dead_code)]
pub struct UserInfo {
    user_id: UserId,
    user_name: String,
    position: HashMap<String, i32>,
    pnl: i64,
}
