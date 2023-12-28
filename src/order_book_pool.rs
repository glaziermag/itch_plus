

// use std::{rc::{Weak, Rc}, cell::RefCell};

// use crate::{OrderBook, MarketManager, Symbol, LevelPool};



// pub trait OrderBookPool<'trading> {
//     fn get_order_book(&self, symbol_id: u64) -> Option<Rc<RefCell<OrderBook>>>;
//     fn add_order_book(&mut self, symbol_id: u64, order_book: OrderBook);
//     // ... other methods related to order book management ...
// }

// impl<'trading> dyn OrderBookPool {
//     pub fn new() -> Box<Self> {
//         Box::new(OrderBook { best_bid: todo!(), best_ask: todo!(), bids: todo!(), asks: todo!(), best_buy_stop: todo!(), best_sell_stop: todo!(), buy_stop: todo!(), sell_stop: todo!(), best_trailing_buy_stop: todo!(), best_trailing_sell_stop: todo!(), trailing_buy_stop: todo!(), trailing_sell_stop: todo!(), last_bid_price: todo!(), last_ask_price: todo!(), matching_bid_price: todo!(), matching_ask_price: todo!(), trailing_bid_price: todo!(), trailing_ask_price: todo!(), level_pool: todo!() })
//     }

//     pub fn create(&mut self, market_manager: Weak<RefCell<MarketManager>>, symbol: Symbol) -> & OrderBook {
//         let order_book = OrderBook::new(market_manager); // Initialize OrderBook
//         self.pool.push(order_book);
//         self.pool.last_mut().unwrap()
//     }

//     pub fn release(&mut self, order_book: OrderBook) {
//         // Logic to 'release' or reuse the OrderBook
//         // This might involve resetting the OrderBook state and keeping it in the pool for future use
//     }
// }