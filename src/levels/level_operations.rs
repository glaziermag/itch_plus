use crate::{order_book::order_book::OrderBook, orders::order::OrderNode, utilities::factory::Factory};

use super::level::{LevelNode, LevelType};
use std::hash::Hash;

trait LevelOperations {
    fn best_ask(&self) -> Option<&LevelNode>;
    fn best_bid(&self) -> Option<&LevelNode>;
    fn get_bid(&mut self, price: u64) -> Option<&LevelNode>;
    fn get_ask(&mut self, price: u64) -> Option<&LevelNode>;
    fn get_next_level(&self, level: LevelNode) -> Option<&LevelNode>;
    fn add_level(&self, order_node: &OrderNode) -> LevelNode;
    fn create_and_insert_level(&self, order_node: &OrderNode, level_type: LevelType) -> Option<&LevelNode>;
    fn delete_level(&self, order_node: &OrderNode) -> LevelNode;
}

impl LevelOperations for OrderBook<'_>{
    fn best_ask(&self) -> Option<&LevelNode> {
        self.best_ask
    }

    fn best_bid(&self) -> Option<&LevelNode> {
        self.best_bid
    } 

    fn get_bid(&mut self, price: u64) -> Option<&LevelNode> {
        self.bids.get(&price)
    }

    fn get_ask(&mut self, price: u64) -> Option<&LevelNode> {
        self.asks.get(&price)
    }

    fn create_and_insert_level<P>(&self, price: P, level_type: LevelType) -> Option<&LevelNode>
        where
            P: Eq + Hash,
        {
        // Create a new price level based on the provided level type
        // Insert the price level into the appropriate collection based on level type
        let node = LevelNode::create(level_type, price);
        match level_type {
            LevelType::Bid => {
                self.bids.insert(price, node);
            },
            LevelType::Ask => {
                self.asks.insert(price, node);
            },
        }
        node
    }

    fn get_next_level(&self, level: LevelNode) -> Option<&LevelNode> {
        if level.is_bid() {
            let mut iter = self.bids.range(..level.price).rev();
            iter.next().map(|(_price, node)| node)
        } else {
            let mut iter = self.asks.range((level.price + 1)..);
            iter.next().map(|(_price, node)| node)
        }
    }

    fn delete_level(&self, order_node: &OrderNode) -> LevelNode {
        let level_node = order_node.level_node;
        if order_node.is_buy() {
            if self.best_bid == level_node {
                // Update the best bid price level
                self.best_bid = if self.best_bid.left != LevelNode::default() {
                    self.best_bid.left
                } else if self.best_bid.parent != LevelNode::default() {
                    self.best_bid.parent
                } else {
                    self.best_bid.right
                };
                self.bids.remove(&level_node.price);
            }
            // Erase the price level from the bid collection
        } else {
            if self.best_ask == level_node {
                // Update the best bid price level
                self.best_ask = if self.best_ask.right != LevelNode::default() {
                    self.best_ask.right
                } else if self.best_ask.parent != LevelNode::default() {
                    self.best_ask.parent
                } else {
                    self.best_ask.left
                };
                self.asks.remove(&level_node.price);
            }
        }
        LevelNode::default()
    }

    fn add_level(&self, order_node: &OrderNode) -> Option<&LevelNode> {

        let level_node: LevelNode;
        if order_node.is_buy() {
            let level_node = self.create_and_insert_level(order_node, LevelType::Bid);

            self.bids.insert(level_node.price, level_node);
            if self.best_bid == LevelNode::default() || level_node.price > self.best_bid.price {
                self.best_bid = level_node
            }
            level_node
        } else {
            let level_node = self.create_and_insert_level(order_node, LevelType::Ask);

            self.bids.insert(level_node.price, level_node);
            if self.best_ask == LevelNode::default() || level_node.price < self.best_ask.price {
                self.best_ask = level_node
            }
            level_node
        }
    }
}
