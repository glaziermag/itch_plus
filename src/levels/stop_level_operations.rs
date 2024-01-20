use crate::{order_book::order_book::OrderBook, orders::order::{OrderNode, OrderType}, utilities::factory::Factory};

use super::level::{LevelNode, LevelType};

trait StopLevelOperations { 
    fn best_buy_stop(&self) -> LevelNode;
    fn best_sell_stop(&self) -> LevelNode;
    fn add_stop_level(&self, order_node: &OrderNode) -> LevelNode;
    fn delete_stop_level(&self, order_node: &OrderNode);
    fn activate_stop_orders_level(&self, order_book: &OrderBook, level_node: LevelNode, stop_price: u64) -> bool;
}   

impl StopLevelOperations for OrderBook<'_> {
    fn best_buy_stop(&self) -> LevelNode {
        self.best_buy_stop
    }

    // Method to get the best sell stop level
    fn best_sell_stop(&self) -> LevelNode {
        self.best_sell_stop
    }

    fn add_stop_level(&self, order_node: &OrderNode) -> LevelNode {
        // Determine the level type and price based on the order node
        // Determine the price and create a level node
        let (price, level_node) = if order_node.is_buy() {
            let level_node = LevelNode::with_price(LevelType::Ask, order_node.stop_price);
            (order_node.stop_price, level_node)
        } else {
            let level_node = LevelNode::with_price(LevelType::Bid, order_node.stop_price);
            (order_node.stop_price, level_node)
        };

        if order_node.is_buy() {
            self.buy_stop.insert(level_node.price, level_node);
            //uninitialized arc pointer
            if self.best_buy_stop == LevelNode::default() || (level_node.price < self.best_buy_stop.price) {
                self.best_buy_stop = level_node;
            }
        } else {
            self.sell_stop.insert(level_node.price, level_node);
            if self.best_sell_stop == LevelNode::default() || (level_node.price < self.best_sell_stop.price) {
                self.best_sell_stop = level_node;
            }
        }
        level_node
    }

    fn delete_stop_level(&self, order_node: &OrderNode) {
        let level_node = order_node.level_node;

        if order_node.is_buy() {
            // Update the best buy stop order price level
            if self.best_buy_stop == level_node {
                self.best_buy_stop = if self.best_buy_stop.right != LevelNode::default() {
                    self.best_buy_stop.right
                } else {
                    self.best_buy_stop.parent
                }
            }
            // Erase the price level from the buy stop orders collection
            self.buy_stop.remove(&level_node.price);
        } else {
            if self.best_sell_stop == level_node {
                // Update the best sell stop order price level
                self.best_sell_stop = if self.best_sell_stop.right != LevelNode::default() {
                    self.best_sell_stop.right
                } else {
                    self.best_sell_stop.parent
                }
            }
            // Erase the price level from the sell stop orders collection
            self.sell_stop.remove(&level_node.price);
        }

        // Release the price level
        // Assuming you have a method in your Rust implementation similar to C++'s Release
       // self.level_pool.release(level_node.price);
    }

    fn activate_stop_orders_level(&self, order_book: &OrderBook, level_node: LevelNode, stop_price: u64) -> bool {

        let mut result = false;
        
        let arbitrage = if level_node.is_bid() {
            stop_price <= level_node.price
        } else {
            stop_price >= level_node.price
        };

        if !arbitrage {
            return false;
        }

        let mut activating_order = level_node.orders.front();
        while let Some(order_node) = activating_order {
            // Clone next_order to avoid borrow_muting issues
            let next_activating_order = order_node.next_mut();

            match order_node.order_type {
                OrderType::Stop | OrderType::TrailingStop => {
                    result |= self.activate_stop_order(order_book, order_node);
                }
                OrderType::StopLimit | OrderType::TrailingStopLimit => {
                    result |= self.activate_stop_limit_order(order_book, order_node);
                }
                _ => {
                    assert!(false, "Unsupported order type!");
                }
            }
            //let next_order = next_activating_order;
            activating_order = next_activating_order;
        }
        result
    }
}