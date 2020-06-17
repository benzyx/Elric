use crate::orderbook::*;
use crate::types::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Exchange {
    books: HashMap<String, OrderBook>,
    users: HashMap<UserId, UserInfo>,
}

#[allow(dead_code)]
impl Exchange {
    pub fn new(symbols: Option<Vec<String>>) -> Exchange {
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
}
