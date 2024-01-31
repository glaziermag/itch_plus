
use std::{collections::BTreeMap, sync::{Mutex, Arc}};
use crate::{levels::{level::{UpdateType, LevelUpdate}, indexing::{LevelNode, RcCell, RcNode, Tree}}, orders::order::{OrderNode, Order}, market_handler::{MarketHandler, Handler}};

#[derive(Debug)]
pub enum OrderBookError {
    OrderNotFound,
    LevelNotFound,
}

pub trait BookOps<'a> {
    fn best_bid(&self) -> Option<RcNode<'a>>;
    fn best_ask(&self) -> Option<RcNode<'a>>;
    fn update_last_price(&mut self, order: Order, price: u64);
    fn update_matching_price(&mut self, order: Order, price: u64);// New methods added to the trait
    fn delete_level(&mut self, order_node: &OrderNode<'a>) -> LevelUpdate<'a>;
    fn subtract_level_volumes(&mut self, level: RcNode<'a>, order_node: &OrderNode<'a>);
    fn unlink_order(&mut self, level: RcNode<'a>, order_node: &OrderNode<'a>);
    fn is_top_of_book(&self, order_node: &OrderNode<'a>) -> bool;
}

impl<'a> BookOps<'a> for OrderBook<'a> {
    fn best_bid(&self) -> Option<RcNode<'a>> {
        self.best_bid
    }
    fn best_ask(&self) -> Option<RcNode<'a>> {
        self.best_ask
    }
    fn update_last_price(&mut self, order: Order, price: u64);
    fn update_matching_price(&mut self, order: Order, price: u64);
}

//impl<'a> BookOps for &OrderBook<'a> {}

#[derive(Default)]
pub struct OrderBook<'a> {

    pub best_bid: Option<RcNode<'a>>,
    pub best_ask: Option<RcNode<'a>>,
    pub bids: Option<RcNode<'a>>,
    pub asks: Option<RcNode<'a>>,

    pub best_buy_stop: Option<RcNode<'a>>,
    pub best_sell_stop: Option<RcNode<'a>>,
    pub buy_stop: Option<RcNode<'a>>,
    pub sell_stop: Option<RcNode<'a>>,

    pub(crate) last_bid_price: u64,
    pub(crate) last_ask_price: u64,
    pub(crate) matching_bid_price: u64,
    pub(crate) matching_ask_price: u64,

    pub best_trailing_buy_stop: Option<RcNode<'a>>,
    pub best_trailing_sell_stop: Option<RcNode<'a>>,
    pub trailing_buy_stop: Option<RcNode<'a>>,
    pub trailing_sell_stop: Option<RcNode<'a>>,
    pub trailing_bid_price: u64,
    pub trailing_ask_price: u64,
}

impl<'a> OrderBook<'_> {

    pub fn new() -> OrderBook<'a> {
        OrderBook {
            best_bid: todo!(),
            best_ask: todo!(),
            bids: todo!(),
            asks: todo!(),
            best_buy_stop: todo!(),
            best_sell_stop: todo!(),
            buy_stop: todo!(),
            sell_stop: todo!(),
            last_bid_price: todo!(),
            last_ask_price: todo!(),
            matching_bid_price: todo!(),
            matching_ask_price: todo!(),
            best_trailing_buy_stop: todo!(),
            best_trailing_sell_stop: todo!(),
            trailing_buy_stop: todo!(),
            trailing_sell_stop: todo!(),
            trailing_bid_price: todo!(),
            trailing_ask_price: todo!(),
        }
    }

    pub fn get_market_trailing_stop_price_ask(&self) -> u64 {
        let last_price = self.last_ask_price;
        let best_price = self.best_ask.map_or(u64::MAX, |ask_node| ask_node.node.value);
        std::cmp::max(last_price, best_price)
    }

    pub fn get_market_trailing_stop_price_bid(&self) -> u64 {
        let last_price = self.last_bid_price;
        let best_price = if self.best_bid.is_some() {
            // remove panicking behavior from code
            self.best_bid.expect("best bid").price
        } else {
            0
        };
        std::cmp::min(last_price, best_price)
    }
    
    pub fn is_top_of_book(&self, order_node: &OrderNode) -> bool {
        if let level = order_node.level_node.level {
            return match order_node.is_buy() {
                true => {
                    // remove panicking behavior from code
                    self.best_bid.expect("best bid").price == level.price
                },
                false => {
                    let best_ask = self.best_ask;
                    // remove panicking behavior from code
                    self.best_bid.expect("best bid").price == level.price
                },
            };
        }
        false
    }

    pub fn update_level<H: Handler>(&mut self, order_book: C,  update: LevelUpdate, market_handler: H) {
        
        match update.update_type {
            UpdateType::Add => H::on_add_level(order_book, &update.update, update.top),
            UpdateType::Update => H::on_update_level(order_book, &update.update, update.top),
            UpdateType::Delete => H::on_delete_level(order_book, &update.update, update.top),
            _ => return,
        };
        H::on_update_order_book(order_book, update.top)
    }

    pub fn on_trailing_stop(&self, order: Order) {
        // Here you would implement the specific logic for handling a trailing stop order
        // For example:
        if order.is_buy() {
            // Handle trailing stop for buy order
            // Update order book, prices, or other states as required
        } else {
            // Handle trailing stop for sell order
            // Update order book, prices, or other states as required
        }
        // Other logic as needed for trailing stops...
    }
}
