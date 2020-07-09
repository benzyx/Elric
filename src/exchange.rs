use crate::messages::*;
use crate::orderbook::*;
use crate::types::*;

use chrono::Local;

use std::collections::HashMap;

#[allow(dead_code)]
pub struct Exchange {
    books: HashMap<Symbol, OrderBook>,
    users: HashMap<UserId, UserInfo>,
}

#[allow(dead_code)]
impl Exchange {
  pub fn new(symbols: Option<Vec<Symbol>>) -> Exchange {
    match symbols {
      Some(sym_vec) => {
        let mut exch = Exchange {
          books: HashMap::new(),
          users: HashMap::new(),
        };
        for sym in sym_vec {
          let sym_copy = sym.clone();
          exch.books.insert(sym, OrderBook::new(sym_copy));
        }
        exch
      }
      None => Exchange {
        books: HashMap::new(),
        users: HashMap::new(),
      },
    }
  }

  pub fn process_limit_order(&mut self, msg: LimitOrderMsg){

    let order = Order {
      side: msg.side,
      price: msg.price,
      qty: msg.qty,
      order_id: msg.order_id,
      time: Local::now(),
    };
    self.books.get_mut(&msg.symbol).unwrap().add_order(order);
  }
}
