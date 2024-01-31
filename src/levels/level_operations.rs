use std::{rc::Rc, cell::RefCell, borrow::Borrow, ptr};

use crate::{order_book::order_book::OrderBook, orders::order::OrderNode};

use super::{level::{Level, LevelType}, indexing::{LevelNode, Tree, RcNode}};

// pub trait Trees<'a> {
//     fn create_and_insert_level(&self, price: u64, level_type: LevelType) -> Option<RcNode<'a>>;
//     fn get_next_level_node(&self, level_node: &LevelNode<'a>) -> Option<RcNode<'a>>;
//     fn delete_level(&self, order_node: &OrderNode<'a>) -> Option<RcNode<'a>>;
//     fn add_level(&self, order_node: &OrderNode<'a>) -> Option<RcNode<'a>>;
// }

impl<'a> OrderBook<'a> {
    pub fn create_and_insert_level<T: Tree<'a>>(&self, price: u64, level_type: LevelType, tree: T) -> Option<Rc<RefCell<LevelNode<'a>>>> {
        // Create a new price level based on the provided level type
        // Insert the price level into the appropriate collection based on level type
        let new_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(level_type, price))));
        match level_type {
            LevelType::Bid => {
                if let Some(bids_root) = self.bids {
                    T::insert(Rc::clone(&bids_root), Rc::clone(&new_node));
                } else {
                    // Handle the case where bids tree is empty
                    self.bids = Some(new_node);
                }
            },
            LevelType::Ask => {
                if let Some(asks_root) = self.asks {
                    T::insert(Rc::clone(&asks_root), Rc::clone(&new_node));
                } else {
                    // Handle the case where bids tree is empty
                    self.asks = Some(new_node);
                }
            },
        }
        Some(new_node)
    }
}

impl<'a> OrderBook<'a> {

    pub fn delete_level(&self, order_node: &OrderNode) {
        // remove panicking behavior from code
        let level_node = order_node.level_node.expect("order node level not retrieved");
        if order_node.is_buy() {
            // remove panicking behavior from code
            let best_bid = self.best_bid.expect("best bid not retrieved");
            let borrowed_best = *best_bid.borrow_mut();
            if ptr::eq(&*best_bid, &*level_node) {
                // Update the best bid price level
                self.best_bid = if borrowed_best.left.is_some() {
                    borrowed_best.left
                } else if borrowed_best.parent.is_some() {
                    borrowed_best.parent
                } else {
                    borrowed_best.right
                };
                (*self.bids.expect("bids not retrieved").borrow_mut()).remove((*level_node.borrow_mut()).price);
            }
            // Erase the price level from the bid collection
        } else {
            // remove panicking behavior from code
            let best_ask: Rc<RefCell<LevelNode<'_>>> = self.best_ask.expect("best bid not retrieved");
            let borrowed_best = *best_ask.borrow_mut();
            if ptr::eq(&*best_ask, &*level_node) {
                // Update the best bid price level
                self.best_ask = if borrowed_best.left.is_some() {
                    borrowed_best.left
                } else if borrowed_best.parent.is_some() {
                    borrowed_best.parent
                } else {
                    borrowed_best.right
                };
                (*self.asks.expect("asks not retrieved").borrow_mut()).remove((*level_node.borrow_mut()).price);
            }
        }
    }

    pub fn add_level<T: Tree<'a>>(&self, order_node: &OrderNode, tree: T) -> Option<RcNode<'a>> {

        let level_node = self.create_and_insert_level(order_node.price, if order_node.is_buy() { LevelType::Bid } else { LevelType::Ask }, tree);
        // remove panicking behavior from code
        let node_borrow = level_node.expect("node creation failed").borrow_mut();
        
        if order_node.is_buy() {
            // remove panicking behavior from code
            if self.best_bid.is_none() || (*node_borrow).price > (*self.best_bid.expect("best bid failed")).borrow().price {
                self.best_bid = level_node
            }
        } else {
            // remove panicking behavior from code
            if self.best_ask.is_none() || (*node_borrow).price < (*self.best_ask.expect("best ask failed")).borrow().price {
                self.best_ask = level_node
            }
        }
        level_node
    }

    pub fn get_next_level_node(&self, level_node: &LevelNode<'a>) -> Option<RcNode<'a>> {
        todo!()
    }
}

impl<'a> OrderBook<'_>{
    pub fn best_ask(&self) -> Option<RcNode<'a>> {
        self.best_ask
    }

    pub fn best_bid(&self) -> Option<RcNode<'a>> {
        self.best_bid
    } 

    pub fn get_bid(&mut self, price: u64) -> Option<Rc<RefCell<LevelNode<'_>>>> {
        (*self.bids.expect("bids not retrieved during get").borrow_mut()).get(price)
    }

    pub fn get_ask(&mut self, price: u64) -> Option<Rc<RefCell<LevelNode<'_>>>>{
        (*self.asks.expect("asks not retrieved during get").borrow_mut()).get(price)
    }
}
