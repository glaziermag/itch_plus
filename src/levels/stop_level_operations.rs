use std::{cell::RefCell, rc::Rc, ptr};

use crate::{order_book::order_book::{OrderBook, BookOps}, orders::order::{OrderNode, OrderType}, market_executors::executor::Execution, references::Convertible};

use super::{level::{Level, LevelType}, indexing::{LevelNode, RcNode, Tree}};
  

impl OrderBook<'_> {
    pub fn best_buy_stop(&self) -> Option<RcNode> {
        self.best_buy_stop
    }

    // Method to get the best sell stop level
    pub fn best_sell_stop(&self) -> Option<RcNode> {
        self.best_sell_stop
    }

    pub fn add_stop_level(&self, order_node: &OrderNode) -> Option<RcNode> {
        // Determine the level type and price based on the order node
        // Determine the price and create a level node
        let level_option = if order_node.is_buy() {
            Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order_node.stop_price))))
        } else {
            Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order_node.stop_price))))
        };

        let level_node = *level_option.borrow_mut();

        if order_node.is_buy() {
            self.buy_stop.insert(level_option);
            // remove panicking behavior from code
            let best_stop = self.best_buy_stop.expect("best stop").borrow();
            if self.best_buy_stop.is_none() || (level_node.level.price < best_stop.level.price) {
                self.best_buy_stop = Some(level_option);
            }
        } else {
            self.sell_stop.insert(level_option);
            // remove panicking behavior from code
            let best_stop = self.best_buy_stop.expect("best stop").borrow();
            if self.best_sell_stop.is_none() || (level_node.level.price < best_stop.level.price) {
                self.best_sell_stop = Some(level_option);
            }
        }
        Some(level_option)
    }

    pub fn delete_stop_level(&self, order_node: &OrderNode) {

        // remove panicking behavior from code
        let level_node = order_node.level_node.expect("order node level node not retrieved");

        if order_node.is_buy() {
            // Update the best buy stop order price level
            // remove panicking behavior from code
            let stop_level = self.best_buy_stop.expect("buy stop not found");
            let borrowed_level = *stop_level.borrow_mut();
            if ptr::eq(&*stop_level, &*level_node) {
                self.best_buy_stop = if borrowed_level.right.is_none() {
                    borrowed_level.right
                } else {
                    borrowed_level.parent
                }   
            }
            // Erase the price level from the buy stop orders collection
            (*stop_level.borrow_mut()).remove(borrowed_level.price);
        } else {
            // remove panicking behavior from code
            let stop_level = self.best_sell_stop.expect("buy stop not found");
            let borrowed_level = *stop_level.borrow_mut();
            if ptr::eq(&*stop_level, &*level_node)  {
                // Update the best sell stop order price level
                self.best_sell_stop = if borrowed_level.right.is_none() {
                    borrowed_level.right
                } else {
                    borrowed_level.parent
                }
            }
            // Erase the price level from the sell stop orders collection
            (*stop_level.borrow_mut()).remove(borrowed_level.price);
        }
    }
}

pub fn activate_stop_orders_level<E, B, C>(order_book: C, mut level: Level, stop_price: u64) -> bool 
where
    E: for<'a> Execution<'a>,
    B: for<'a> BookOps<'a>,
    C: Convertible<B>,
{

    let mut result = false;
    
    let arbitrage = if level.is_bid() {
        stop_price <= level.price
    } else {
        stop_price >= level.price
    };

    if !arbitrage {
        return false;
    }

    let mut activating_order = level.orders.front_mut();
    while let Some(order_node) = activating_order {
        // Clone next_order to avoid borrow_muting issues
        let next_activating_order = order_node.next_mut();

        match order_node.order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= E::activate_stop_order(order_book, order_node);
            }
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= E::activate_stop_limit_order(order_book, order_node);
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