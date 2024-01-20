use std::cmp::{max, min};

use crate::{levels::level::{LevelNode, LevelType}, orders::order::{Order, OrderType}};

use super::order_book::OrderBook;

pub trait PriceOperations {
    fn reset_matching_price(&self);
    fn get_market_ask_price(&self) -> u64;
    fn get_market_bid_price(&self) -> u64;
    fn update_last_price(&self, order: Order, price: u64);
    fn update_matching_price(&self, order: Order, price: u64);
    fn recalculate_trailing_stop_price(&self, order_book: &OrderBook, level_node: LevelNode);
    fn calculate_trailing_stop_price(&self, order: Order) -> u64;
}

impl PriceOperations for OrderBook<'_> {

    fn reset_matching_price(&self) {
        self.matching_bid_price = 0;
        self.matching_ask_price = u64::MAX;
    }

    fn get_market_ask_price(&self) -> u64 {
        let best_price = if self.best_ask != LevelNode::default() {
            self.best_ask.price
        } else {
            u64::MAX
        };
        min(best_price, self.matching_ask_price)
    }

    fn get_market_bid_price(&self) -> u64 {
        let best_price = if self.best_bid != LevelNode::default() {
            self.best_bid.price
        } else {
            0
        };
        max(best_price, self.matching_bid_price)
    }

    fn update_last_price(&self, order: Order, price: u64) {
        if order.is_buy() {
            self.last_bid_price = price;
        } else {
            self.last_ask_price = price;
        }
    }

    fn update_matching_price(&self, order: Order, price: u64) {
        if order.is_buy() {
            self.matching_bid_price = price;
        } else {
            self.matching_ask_price = price;
        }
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
}