use std::{collections::HashMap};

use crate::{order_book::order_book::OrderBook, market_handler::MarketHandler, levels::level::LevelNode, orders::order::Order};


pub struct MarketExecutor {
    pub orders: HashMap<u64, Order>,
    matching: bool,
    recursive: bool
}
pub trait Executor{}

impl Executor for MarketExecutor {}

pub trait MatchChain<E: Executor, M: MarketHandler> {
    fn execute_matching_chain(executor: &E, order_book: &OrderBook, level_node: LevelNode, price: u64, volume: u64, market_handler: &M);
}

impl MarketExecutor {
    pub fn get_order(&self, order_id: u64) -> Option<&Order> {
        self.get_order(order_id)
    }

    pub fn remove_order(&mut self, order_id: u64) -> Option<Order> {
        self.orders.remove(&order_id)
    }
}

impl<E: Executor, M: MarketHandler> MatchChain<E, M> for MarketExecutor {

    fn execute_matching_chain(executor: &E, mut order_book: &OrderBook, level_node: LevelNode, price: u64, mut volume: u64, market_handler: &M) {
    
        let mut level_node = Some(level_node);
    
        while volume > 0 {
            if let Some(ref mut current_level_node) = level_node {
                let mut executing_order = current_level_node.orders.front();
    
                while volume > 0 {
                    if let Some(order_node) = executing_order {
                        let quantity = if order_node.is_aon() {
                            order_node.leaves_quantity
                        } else {
                            std::cmp::min(order_node.leaves_quantity, volume)
                        };
    
                        market_handler.on_execute_order(&order_node.order, price, quantity);
                        // Switch to the next price level
                        order_book.update_last_price(order_node.order, price);
                        order_book.update_matching_price(order_node.order, price);
                        
                        order_node.executed_quantity += quantity;
                        // Reduce the executing order in the order book
                        executor.reduce_order(order_node.id, quantity, true, false);
    
                        volume -= quantity;
                        executing_order = order_node.next_mut();
                    } else {
                        break;
                    }
                }// Assuming `get_next_level` returns an LevelNode
                if let Some(next_level_node) = order_book.get_next_level(current_level_node) {
                    level_node = Some(next_level_node);
                    
                } else {
                }
                
            } else {
                break;
            }
        }
    }
}
