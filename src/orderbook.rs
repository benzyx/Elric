use std::collections::hash_map::HashMap;
use std::collections::BTreeMap;
use chrono::Local;
use crate::event::*;

// Really, a resting order on the book.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Order {
    side: Side,
    price: Price,
    qty: Qty,
    order_id: OrderId,
    time: Timestamp,
}

type OrderMap = BTreeMap<Price, BTreeMap<Timestamp, OrderId>>;
type OrderSet = HashMap<OrderId, Order>;

// Orderbook for a single symbol.
#[allow(dead_code)]
pub struct OrderBook {
    symbol: String,
    buy_order_tree: OrderMap,
    sell_order_tree: OrderMap,
    buy_order_set: OrderSet,
    sell_order_set: OrderSet,
}

#[allow(dead_code)]
impl OrderBook {
    pub fn new(symbol: String) -> OrderBook {
        OrderBook {
            symbol: symbol,
            buy_order_tree: OrderMap::new(),
            sell_order_tree: OrderMap::new(),
            buy_order_set: OrderSet::new(),
            sell_order_set: OrderSet::new(),
        }
    }

    pub fn best_bid(&self) -> Option<Price> {
        self.buy_order_tree.iter().rev().next().map(|(price, _time_map)| {
            price.clone()
        })

    }

    pub fn best_ask(&self) -> Option<Price> {
        self.sell_order_tree.iter().next().map(|(price, _time_map)| {
            price.clone()
        })
    }

    fn get_side_objs(&mut self, side: &Side) -> (&mut OrderMap, &mut OrderSet) {
        match side {
            Side::Buy => {
                return (&mut self.buy_order_tree, &mut self.buy_order_set);
            }
            Side::Sell => {
                return (&mut self.sell_order_tree, &mut self.sell_order_set);
            }
        }
    }

    pub fn add_order(&mut self, order: Order) -> Vec<Event> {
        let (book, order_set) = self.get_side_objs(&order.side);
        if !book.contains_key(&order.price) {
            book.insert(order.price, BTreeMap::new());
        }
        book.get_mut(&order.price).unwrap().insert(order.time, order.order_id);

        order_set.insert(order.order_id, order);

        self.process_trades()
    }

    pub fn remove_order(&mut self, order_id: OrderId) {
        let order: Order = if self.buy_order_set.contains_key(&order_id) {
            self.buy_order_set.remove(&order_id).unwrap()
        }
        else {
            self.sell_order_set.remove(&order_id).unwrap()
        };

        let (book, _order_set) = self.get_side_objs(&order.side);

        if !book.contains_key(&order.price) {
            return;
        }

        let time_map = book.get_mut(&order.price).unwrap();

        time_map.remove(&order.time);
        if time_map.is_empty() {
            book.remove(&order.price);
        }
    }

    fn price_cross(&self) -> bool {
        let best_bid = self.best_bid();
        let best_ask = self.best_ask();
        if best_bid.is_none() || best_ask.is_none() {
            return false;
        }
        return best_bid.unwrap() > best_ask.unwrap();
    }

    fn process_trades(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        while self.price_cross() {

            let (bid_timestamp, bid_id) : (Timestamp, OrderId);
            let (ask_timestamp, ask_id) : (Timestamp, OrderId);
            {
                let (_, level_bids) = self.buy_order_tree.iter().rev().next().unwrap();
                let (_, level_asks) = self.sell_order_tree.iter().next().unwrap();
                
                // Get earliest bid and ask order_ids at current price level.
                let (bid_timestamp_ref, bid_id_ref) : (&Timestamp, &OrderId) = level_bids.iter().next().unwrap();
                let (ask_timestamp_ref, ask_id_ref) : (&Timestamp, &OrderId)= level_asks.iter().next().unwrap();
                bid_timestamp = *bid_timestamp_ref;
                bid_id = *bid_id_ref;
                ask_timestamp = *ask_timestamp_ref;
                ask_id = *ask_id_ref;
            }
            
            let bid_order = self.buy_order_set.get_mut(&bid_id).unwrap();
            let ask_order = self.sell_order_set.get_mut(&ask_id).unwrap();

            let trade_price = if bid_timestamp < ask_timestamp {
                bid_order.price
            } 
            else {
                ask_order.price
            };
            /*
                We need to delete or modify the orders being matched.
            */
            let executed_qty;
            // Delete both orders.
            if bid_order.qty == ask_order.qty {
                executed_qty = bid_order.qty;

                self.remove_order(bid_id);
                self.remove_order(ask_id);
            }
            // Only remove ask order, modify qty on bid order.
            else if bid_order.qty > ask_order.qty {
                executed_qty = ask_order.qty;
                // Modify bid order to have lower quantity.
                bid_order.qty -= executed_qty;
                // Remove ask order
                self.remove_order(ask_id);                
            }
            else {
                executed_qty = bid_order.qty;
                // Modify ask order to have lower quantity.
                ask_order.qty -= executed_qty;
                // Remove bid order.
                self.remove_order(bid_id);
            }
            let event = Event::Executed{
                bid_id: bid_id.clone(),
                ask_id: ask_id.clone(),
                price: trade_price.clone(),
                qty: executed_qty,
                time: Local::now(),
            };
            
            events.push(event);
        }
        events
    }

    pub fn print_book(&self) {

    }
}


#[cfg(test)]
mod test {
    use crate::orderbook::*;
    

    #[test]
    fn basics() {
        let mut orderbook = OrderBook::new(String::from("GOOG"));

        assert_eq!(orderbook.best_bid(), None);
        assert_eq!(orderbook.best_ask(), None);

        assert_eq!(orderbook.add_order(Order {
            side: Side::Buy,
            price: 10500,
            qty: 100,
            order_id: 15,
            time: Local::now(),
        }).len(), 0);

        assert_eq!(orderbook.best_bid(), Some(10500));
        assert_eq!(orderbook.best_ask(), None);

        assert_eq!(orderbook.add_order(Order {
            side: Side::Sell,
            price: 11000,
            qty: 100,
            order_id: 15,
            time: Local::now(),
        }).len(), 0);

        assert_eq!(orderbook.best_bid(), Some(10500));
        assert_eq!(orderbook.best_ask(), Some(11000));

    }
}