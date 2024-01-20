
use crate::{order_book::order_book::OrderBook, orders::order::OrderNode};

use super::level::{LevelNode, LevelType};

trait TrailingStopLevelOperations { 
    fn best_trailing_buy_stop(&self) -> LevelNode;
    fn best_trailing_sell_stop(&self) -> LevelNode;
    fn get_next_trailing_stop_level(&self, level_node: LevelNode) -> LevelNode;
    fn delete_trailing_stop_level(&self, order_node: &OrderNode) -> LevelNode;
    fn add_trailing_stop_level(&self, order_node: &OrderNode) -> LevelNode;
    fn get_trailing_buy_stop_level(&mut self, price: u64) -> Option<&LevelNode>;
    fn get_trailing_sell_stop_level(&mut self, price: u64) -> Option<&LevelNode>;
}

impl TrailingStopLevelOperations for OrderBook<'_> {

    // Method to get the best trailing buy stop level
    fn best_trailing_buy_stop(&self) -> LevelNode {
        self.best_trailing_buy_stop
    }

    // Method to get the best trailing sell stop level
    fn best_trailing_sell_stop(&self) -> LevelNode {
        self.best_trailing_sell_stop
    }

    fn get_trailing_buy_stop_level(&mut self, price: u64) -> Option<&LevelNode> {
        self.trailing_buy_stop.get(&price)
    }

    // Method to get the trailing sell stop level
    fn get_trailing_sell_stop_level(&mut self, price: u64) -> Option<&LevelNode> {
        self.trailing_sell_stop.get(&price)
    }

    fn get_next_trailing_stop_level(&self, level_node: LevelNode) -> LevelNode {
            
        if level_node.is_bid() {
            // Find the next level in reverse order in _trailing_sell_stop
            let var= self.trailing_sell_stop
                .range(..level_node.price).rev() // Iterate in reverse up to the current price
                .next()
                .expect("next level")
                .1;
            *var  // Return the node if found
        } else {
            // Find the next level in normal order in _trailing_buy_stop
            let var = self.trailing_buy_stop
                .range((level_node.price + 1)..) // Iterate starting from just above the current price
                .next()
                .expect("next level")                // Get the next element
                .1;
            *var
        }
    }

    fn delete_trailing_stop_level(&self, order_node: &OrderNode) -> LevelNode {
        
        let level_node = order_node.level_node;
        if order_node.is_buy() {
            // Update the best trailing buy stop order price level
            if *level_node == *self.best_trailing_buy_stop() {
                self.best_trailing_buy_stop = if self.best_trailing_buy_stop.right != LevelNode::default() {
                    self.best_trailing_buy_stop.right
                } else {
                    self.best_trailing_buy_stop.parent
                }
            }
            // Erase the price level from the trailing buy stop orders collection
            self.trailing_buy_stop.remove(&level_node.price);
        } else {
            // Update the best trailing sell stop order price level
            if *level_node == *self.best_trailing_sell_stop() {
                self.best_trailing_sell_stop = if self.best_trailing_sell_stop.left != LevelNode::default() {
                    self.best_trailing_sell_stop.left
                } else {
                    self.best_trailing_sell_stop.parent
                }
            }
            // Erase the price level from the trailing sell stop orders collection
            self.trailing_sell_stop.remove(&level_node.price);
        }
        // Release the price level
        self.level_pool.release(level_node.price)
    }

    fn add_trailing_stop_level(&self, order_node: &OrderNode) -> LevelNode {

        let (price, level_node) = if order_node.is_buy() {
            let level_node = LevelNode::create(LevelType::Ask, order_node.stop_price);
            (order_node.stop_price, level_node)
        } else {
            let level_node = LevelNode::create(LevelType::Bid, order_node.stop_price);
            (order_node.stop_price, level_node)
        };
        
        if order_node.is_buy() {
            self.trailing_buy_stop.insert(level_node.price, level_node);
            // Update the best trailing buy stop order price level
            if self.best_trailing_buy_stop == LevelNode::default() || (level_node.price < self.best_trailing_buy_stop().price) {
                self.best_trailing_buy_stop = level_node;
            }
        } else {
            self.trailing_sell_stop.insert(level_node.price, level_node);
            // Update the best trailing sell stop order price level
            if self.best_trailing_sell_stop == LevelNode::default() || (level_node.price < self.best_trailing_sell_stop().price) {
                self.best_trailing_sell_stop = level_node;
            }
        }
        level_node
    }
}