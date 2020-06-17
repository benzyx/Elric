use chrono::{DateTime, Local};
use std::collections::HashMap;
pub type OrderId = i64;
pub type Price = i64;
pub type Qty = i64;
pub type Timestamp = DateTime<Local>;
pub type UserId = i64;

pub struct UserInfo {
    user_id: UserId,
    user_name: String,
    position: HashMap<String, i32>,
    pnl: i64,
}
