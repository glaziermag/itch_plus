
use std::{ptr, cell::RefCell, rc::Rc};

use crate::{order_book::order_book::OrderBook, orders::order::OrderNode};

use super::{level::{Level, LevelType}, indexing::{LevelNode, RcNode, Tree}};

impl OrderBook<'_> {

    // Method to get the best trailing buy stop level
    pub fn best_trailing_buy_stop(&self) -> Option<RcNode> {
        self.best_trailing_buy_stop
    }

    // Method to get the best trailing sell stop level
    pub fn best_trailing_sell_stop(&self) -> Option<RcNode> {
        self.best_trailing_sell_stop
    }

    pub fn get_trailing_buy_stop_level(&mut self, price: u64) -> Option<RcNode> {
        (*self.trailing_buy_stop.expect("best trailing buy stop failed").borrow_mut()).get(price)
    }

    // Method to get the trailing sell stop level
    pub fn get_trailing_sell_stop_level(&mut self, price: u64) -> Option<RcNode> {
        (*self.trailing_sell_stop.expect("best trailing sell stop failed").borrow_mut()).get(price)
    }

    pub fn get_next_trailing_stop_level<T: for<'a> Tree<'a>>(&self, level_node: RcNode) -> Option<RcNode> {
            
        if (*level_node.borrow_mut()).is_bid() {
            // Find the next level in reverse order in _trailing_sell_stop
            <LevelNode as Tree>::get_next_lower_level(self.trailing_sell_stop.expect("best trailing sell stop failed"))
        } else {
            // Find the next level in normal order in _trailing_buy_stop
            <LevelNode as Tree>::get_next_higher_level(self.trailing_buy_stop.expect("best trailing buy stop failed"))
        }
    }

    pub fn delete_trailing_stop_level(&self, order_node: &OrderNode) {

        // remove panicking behavior from code
        let level_node = order_node.level_node.expect("level node not retrieved");
        
        if order_node.is_buy() {
            // Update the best trailing buy stop order price level
            // remove panicking behavior from code
            let best_stop = self.best_trailing_buy_stop.expect("best stop not retrieved");
            if ptr::eq(&*best_stop, &*level_node) {
                let borrow_stop = best_stop.borrow();
                self.best_trailing_buy_stop = if borrow_stop.right.is_none() {
                    borrow_stop.right.clone()
                } else {
                    borrow_stop.parent.clone()
                }
            }
            // Erase the price level from the trailing buy stop orders collection
            (*self.trailing_buy_stop.expect("best trailing buy stop failed").borrow_mut()).remove((*level_node.borrow()).price);
        } else {
            // Update the best trailing sell stop order price level
            // remove panicking behavior from code
            let best_stop = self.best_trailing_sell_stop.expect("best stop not retrieved");
            if ptr::eq(&*best_stop, &*level_node) {
                let borrow_stop = best_stop.borrow();
                self.best_trailing_sell_stop = if borrow_stop.left.is_none() {
                    borrow_stop.left.clone()
                } else {
                    borrow_stop.parent.clone()
                }
            }
            // Erase the price level from the trailing sell stop orders collection
            (*self.trailing_sell_stop.expect("best trailing sell stop failed").borrow_mut()).remove((*level_node.borrow()).price);
        }
        // Release the price level
       // self.level_pool.release(level_node.price)
    }

    pub fn add_trailing_stop_level(&self, order_node: &OrderNode) -> Option<RcNode> {

        let (price, level_node) = if order_node.is_buy() {
            let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order_node.stop_price))));
            (order_node.stop_price, level_node)
        } else {
            let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order_node.stop_price))));
            (order_node.stop_price, level_node)
        };
        
        if order_node.is_buy() {
            self.trailing_buy_stop.insert(level_node);
            // Update the best trailing buy stop order price level
            if self.best_trailing_buy_stop.is_none() || ((*level_node.borrow()).price < (*self.best_trailing_buy_stop.expect("best trailing sell stop failed").borrow()).price) {
                self.best_trailing_buy_stop = Some(level_node);
            }
        } else {
            self.trailing_sell_stop.insert(level_node);
            // Update the best trailing sell stop order price level
            if self.best_trailing_sell_stop.is_none() || ((*level_node.borrow()).price < (*self.best_trailing_sell_stop.expect("best trailing sell stop failed").borrow()).price) {
                self.best_trailing_sell_stop = Some(level_node);
            }
        }
        Some(level_node)
    }
}