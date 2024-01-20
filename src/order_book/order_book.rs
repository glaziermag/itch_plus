
use std::{collections::BTreeMap, sync::{Mutex, Arc}};
use crate::{levels::level::{LevelType, UpdateType, LevelUpdate}, orders::order::{OrderNode, Order, OrderType}};
use crate::levels::level::LevelNode;

#[derive(Debug)]
pub enum OrderBookError {
    OrderNotFound,
    LevelNotFound,
}

#[derive(Debug, Default)]
pub struct OrderBook<'a> {

    pub best_bid: Option<&'a LevelNode>,
    pub best_ask: Option<&'a LevelNode>,
    pub bids: BTreeMap<u64, LevelNode>,
    pub asks: BTreeMap<u64, LevelNode>,

    pub best_buy_stop: Option<&'a LevelNode>,
    pub best_sell_stop: Option<&'a LevelNode>,
    pub buy_stop: BTreeMap<u64, LevelNode>,
    pub sell_stop: BTreeMap<u64, LevelNode>,

    // Market last and trailing prices
    pub(crate) last_bid_price: u64,
    pub(crate) last_ask_price: u64,
    pub(crate) matching_bid_price: u64,
    pub(crate) matching_ask_price: u64,

    pub best_trailing_buy_stop: Option<&'a LevelNode>,
    pub best_trailing_sell_stop: Option<&'a LevelNode>,
    pub trailing_buy_stop: BTreeMap<u64, LevelNode>,
    pub trailing_sell_stop: BTreeMap<u64, LevelNode>,
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
            self.best_bid.price
        } else {
            0
        };
        std::cmp::min(last_price, best_price)
    }
    
    pub fn is_top_of_book(&self, order_node: &OrderNode) -> bool {
        if let level = order_node.level_node.level {
            return match order_node.is_buy() {
                true => {
                    self.best_bid.price == level.price
                },
                false => {
                    let best_ask = self.best_ask;
                    self.best_ask.price == level.price
                },
            };
        }
        false
    }

    pub fn update_level(&mut self, order_book: OrderBook, update: LevelUpdate) {
        
        match update.update_type {
            UpdateType::Add => self.market_handler.on_add_level(order_book, &update.update, update.top),
            UpdateType::Update => self.market_handler.on_update_level(order_book, &update.update, update.top),
            UpdateType::Delete => self.market_handler.on_delete_level(order_book, &update.update, update.top),
            _ => return,
        };
        self.market_handler.on_update_order_book(order_book, update.top)
    }

    fn calculate_trailing_stop_price(&self, order: Order) -> u64 {
        // Get the current market price
        let market_price = if order.is_buy() {
            self.get_market_trailing_stop_price_ask()
        } else {
            self.get_market_trailing_stop_price_bid()
        };
        let mut trailing_distance = order.trailing_distance as i64;
        let mut trailing_step = order.trailing_step as i64;

        // Convert percentage trailing values into absolute ones
        if trailing_distance < 0 {
            trailing_distance = - trailing_distance * market_price as i64 / 10000;
            trailing_step = - trailing_step * market_price as i64 / 10000;
        }

        let old_price = order.stop_price;

        if order.is_buy() {
            // Calculate a new stop price
            let new_price = market_price.checked_add(trailing_distance as u64).unwrap_or(u64::MAX);

            // If the new price is better and we get through the trailing step
            if new_price < old_price && (old_price - new_price) >= trailing_step as u64 {
                return new_price;
            }
        } else {
            // Calculate a new stop price
            let new_price = market_price.checked_sub(trailing_distance as u64).unwrap_or(0);

            // If the new price is better and we get through the trailing step
            if new_price > old_price && (new_price - old_price) >= trailing_step as u64 {
                return new_price;
            }
        }
        old_price
    }

    fn recalculate_trailing_stop_price(&self, mut order_book: &OrderBook, level_node: LevelNode) {
        let mut new_trailing_price;

        // Skip recalculation if market price goes in the wrong direction
        match level_node.level_type {
            Some(LevelType::Ask) => {
                let old_trailing_price = order_book.trailing_ask_price;
                new_trailing_price = order_book.get_market_trailing_stop_price_ask();
                if new_trailing_price >= old_trailing_price {
                    return;
                }
                order_book.trailing_ask_price = new_trailing_price;
            },
            Some(LevelType::Bid) => {
                let old_trailing_price = order_book.trailing_bid_price;
                new_trailing_price = order_book.get_market_trailing_stop_price_bid();
                if new_trailing_price <= old_trailing_price {
                    return;
                }
                order_book.trailing_bid_price = new_trailing_price;
            },
            None => todo!(),
        }

        // Recalculate trailing stop self.orders
        let mut current = match level_node.level_type {
            LevelType::Ask => {
                order_book.best_trailing_buy_stop
            },
            LevelType::Bid => {
                order_book.best_trailing_sell_stop
            }
        };

        let mut previous: Option<LevelNode> = None;
        let mut current = Some(current);

        while let Some(ref current_level) = current {
            let mut recalculated = false;
            let mut node = current_level.orders.front_mut();

            while let Some(order_node) = node {
                let next_order = order_node.next_mut();
                let old_stop_price = order_node.stop_price;
                let new_stop_price = order_book.calculate_trailing_stop_price(order_node.order);

                // Update and re-add order if stop price changed
                if new_stop_price != old_stop_price {
                    order_book.delete_trailing_stop_order(order_node);
                    // Update stop price based on order type
                    match order_node.order_type {
                        OrderType::TrailingStop => order_node.stop_price = new_stop_price,
                        OrderType::TrailingStopLimit => {
                            let diff = order_node.price - order_node.stop_price;
                            order_node.stop_price = new_stop_price;
                            order_node.price = new_stop_price + diff;
                        },
                        _ => panic!("Unsupported order type!"),
                    }
                    // market_handler.on_update_order(&order_node.order);
                    order_book.add_trailing_stop_order(order_node);
                    recalculated = true;
                }
                node = next_order;
            }

            if recalculated {
                let current = if let Some(ref prev) = previous {
                    Some(prev) 
                } else if level_node.level_type == Some(LevelType::Ask) {
                    Some(order_book.best_trailing_buy_stop)
                } else {
                    Some(order_book.best_trailing_sell_stop)
                };
            } else {
                previous = current;
                
                current = Some(order_book.get_next_trailing_stop_level(current_level));
            }
        }
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
