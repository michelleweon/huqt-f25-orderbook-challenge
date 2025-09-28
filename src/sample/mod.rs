// importing dependencies from types.rs and stdlib modules to current crate
use crate::types::{OrderBook, Side};
use std::collections::{BTreeMap, HashMap, VecDeque};

// derive macro to implement Clone and Debug traits for Order struct
#[derive(Clone, Debug)]
// defining Order struct with id, side, size, and price fields
struct Order {
    id: u64,
    side: Side,
    size: i64,
    price: i64,
}

// defining OrderBookImpl struct with orders, bids, asks, total_volume, and total_notional_volume fields
pub struct OrderBookImpl {
    orders: HashMap<u64, Order>,
    // BTreeMap to automatically sort price-ordered books in O(log n) time
    // VecDeque to add and remove elements from both ends in O(1) time with FIFO
    bids: BTreeMap<i64, VecDeque<Order>>,
    asks: BTreeMap<i64, VecDeque<Order>>,
    total_volume: i64,
    total_notional_volume: i64,
}

// trait implementation for OrderBookImpl
impl OrderBook for OrderBookImpl {
    // new_book function to create a new OrderBookImpl and initialize its default fields
    fn new_book() -> Self {
        Self {
            orders: HashMap::new(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            total_volume: 0,
            total_notional_volume: 0,
        }
    }

    // add_order function to add a new order to the orderbook and match it with existing orders
    fn add_order(&mut self, id: u64, side: Side, size: i64, price: i64) {
        if size <= 0 {
            return;
        }

        let mut remaining_size = size;

        match side {
            Side::Bid => {
                self.match_against_asks(&mut remaining_size, price);
            }
            Side::Ask => {
                self.match_against_bids(&mut remaining_size, price);
            }
        }

        if remaining_size > 0 {
            let order = Order { id, side, size: remaining_size, price };
            self.orders.insert(id, order.clone());
            
            match side {
                Side::Bid => {
                    self.bids.entry(price).or_insert_with(VecDeque::new).push_back(order);
                }
                Side::Ask => {
                    self.asks.entry(price).or_insert_with(VecDeque::new).push_back(order);
                }
            }
        }
    }

    // cancel_order function to cancel an order with the specified id
    fn cancel_order(&mut self, id: u64) {
        if let Some(order) = self.orders.remove(&id) {
            match order.side {
                Side::Bid => {
                    if let Some(orders_at_price) = self.bids.get_mut(&order.price) {
                        if let Some(pos) = orders_at_price.iter().position(|o| o.id == id) {
                            orders_at_price.remove(pos);
                        }
                        if orders_at_price.is_empty() {
                            self.bids.remove(&order.price);
                        }
                    }
                }
                Side::Ask => {
                    if let Some(orders_at_price) = self.asks.get_mut(&order.price) {
                        if let Some(pos) = orders_at_price.iter().position(|o| o.id == id) {
                            orders_at_price.remove(pos);
                        }
                        if orders_at_price.is_empty() {
                            self.asks.remove(&order.price);
                        }
                    }
                }
            }
        }
    }

    // get_vol function to return the total volume traded since creation
    fn get_vol(&self) -> i64 {
        self.total_volume
    }

    // get_ntnl_vol function to return the total notional volume traded since creation
    fn get_ntnl_vol(&self) -> i64 {
        self.total_notional_volume
    }
}

impl OrderBookImpl {
    // match_against_asks function to match against asks with the specified bid price
    fn match_against_asks(&mut self, remaining_size: &mut i64, bid_price: i64) {
        let ask_prices: Vec<i64> = self.asks.keys().cloned().collect();
        
        for ask_price in ask_prices {
            if ask_price > bid_price || *remaining_size <= 0 {
                break;
            }
            
            if let Some(orders_at_price) = self.asks.get_mut(&ask_price) {
                while !orders_at_price.is_empty() && *remaining_size > 0 {
                    if let Some(mut ask_order) = orders_at_price.pop_front() {
                        let trade_size = (*remaining_size).min(ask_order.size);
                        
                        self.total_volume += trade_size;
                        self.total_notional_volume += trade_size * ask_price;
                        *remaining_size -= trade_size;
                        ask_order.size -= trade_size;
                        
                        if ask_order.size > 0 {
                            orders_at_price.push_front(ask_order);
                            break;
                        } else {
                            self.orders.remove(&ask_order.id);
                        }
                    }
                }
                
                if orders_at_price.is_empty() {
                    self.asks.remove(&ask_price);
                }
            }
        }
    }

    // match_against_bids function to match against bids with the specified ask price
    fn match_against_bids(&mut self, remaining_size: &mut i64, ask_price: i64) {
        let bid_prices: Vec<i64> = self.bids.keys().rev().cloned().collect();
        
        for bid_price in bid_prices {
            if bid_price < ask_price || *remaining_size <= 0 {
                break;
            }
            
            if let Some(orders_at_price) = self.bids.get_mut(&bid_price) {
                while !orders_at_price.is_empty() && *remaining_size > 0 {
                    if let Some(mut bid_order) = orders_at_price.pop_front() {
                        let trade_size = (*remaining_size).min(bid_order.size);
                        
                        self.total_volume += trade_size;
                        self.total_notional_volume += trade_size * bid_price;
                        *remaining_size -= trade_size;
                        bid_order.size -= trade_size;
                        
                        if bid_order.size > 0 {
                            orders_at_price.push_front(bid_order);
                            break;
                        } else {
                            self.orders.remove(&bid_order.id);
                        }
                    }
                }
                
                if orders_at_price.is_empty() {
                    self.bids.remove(&bid_price);
                }
            }
        }
    }
}