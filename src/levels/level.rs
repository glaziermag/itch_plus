

use std::{collections::LinkedList, cmp::Ordering};

use orders::order::OrderNode;

use crate::orders;

use super::indexing::{LevelNode, RcNode};

pub struct Level<'a> {
    pub price: u64,
    pub tree_node: RcNode<'a>,
    pub total_volume: u64,
    pub hidden_volume: u64,
    pub visible_volume: u64,
    pub(crate) orders: LinkedList<OrderNode<'a>>,
    pub level_type: LevelType,
}

impl<'a> From<Level<'a>> for LevelNode<'a> {
    fn from(level: Level<'a>) -> Self {
        LevelNode {
            level,
            parent: None,
            left: None,
            right: None,
        }
    }
}


impl<'a> Level<'a> {
    /// Creates a new `Level` instance.
    /// 
    /// The method returns `Pin<Box<Self>>` because `Level` instances might contain
    /// self-referential data (e.g., parent, left, right raw pointers). `Pin` guarantees
    /// that the `Level` instance will not be moved in memory, which is crucial for 
    /// maintaining the validity of self-referential pointers.

    pub fn with_price(level_type: LevelType, price: u64) -> Self {
        Level {
            price,
            total_volume: 0,   // Default value
            hidden_volume: 0,  // Default value
            visible_volume: 0, // Default value
            orders: LinkedList::new(), // Initialize with an empty LinkedList
            level_type,
            tree_node: todo!(), // Default value
            // parent: None,   // No parent initially
            // left: None,     // No left child initially
            // right: None,    // No right child initially
            // _pinned: PhantomPinned,
        }
    }

    pub fn is_bid(&self) -> bool {
        self.level_type == LevelType::Bid
    }

    pub fn is_ask(&self) -> bool {
        self.level_type == LevelType::Ask
    }
    
    pub fn subtract_volumes(mut level: Level, order_node: &OrderNode) {
        level.total_volume -= order_node.order.leaves_quantity();
        level.hidden_volume -= order_node.order.hidden_quantity();
        level.visible_volume -= order_node.order.visible_quantity();
    }

    pub fn add_volumes(mut level: Level, order_node: &OrderNode) {
        level.total_volume += order_node.order.leaves_quantity();
        level.hidden_volume += order_node.order.hidden_quantity();
        level.visible_volume += order_node.order.visible_quantity();
    }

    // pub fn link_order(&self, mut level: Level, order_node: &OrderNode) {
    //     level.orders.pop_current(&order_node); // push_back for LinkedList
    //     level.orders += 1;
    // }

    pub fn unlink_order(&self, mut level: Level, order_node: &OrderNode) {
        level.orders.pop_current(&order_node); 
    }
}


pub enum UpdateType {
    Add,
    Update,
    Delete,
}

pub struct LevelUpdate<'a> {
    pub(crate)update_type: UpdateType,
    pub(crate)update: Level<'a>, 
    pub(crate)top: bool,
}

pub trait PopCurrent <T>{
    fn pop_current(&mut self, value: &T) -> Option<T>;
}


impl<'a> From<Level<'a>> for *mut Level<'a> {
    fn from(value: Level<'a>) -> Self {
        Box::into_raw(Box::new(value))
    }
}


impl<T: PartialEq> PopCurrent<T> for LinkedList<T> {
    fn pop_current(&mut self, value: &T) -> Option<T> {
        let mut new_list = LinkedList::new();
        let mut removed_item = None;

        while let Some(item) = self.pop_front() {
            if item == *value && removed_item.is_none() {
                removed_item = Some(item);
            } else {
                new_list.push_back(item);
            }
        }
        *self = new_list;
        removed_item
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelType {
    Bid,
    Ask,
}


impl<'a> PartialEq for Level<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Defer to Level's implementation of PartialEq
        self.price.eq(&other.price)
    }
}

impl<'a> Eq for Level<'a> {}

impl<'a> PartialOrd for Level<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Defer to Level's implementation of PartialOrd
        self.price.partial_cmp(&other.price)
    }
}

impl<'a> Level<'a> {

    pub fn new_as_ptr(price: u64, level: Level<'a>, parent: Level<'a>) -> Self {
        Level {
            orders: todo!(),
            price,
            total_volume: todo!(),
            hidden_volume: todo!(),
            visible_volume: todo!(),
            level_type: todo!(),
            tree_node: todo!(),
            // parent: todo!(),
            // left: todo!(),
            // right: todo!(),
            // _pinned: PhantomPinned,
        }
    }

    pub fn new_from_order_node(price: u64, level_type: LevelType) -> Self {
        Level {
            orders: todo!(),
            price,
            total_volume: todo!(),
            hidden_volume: todo!(),
            visible_volume: todo!(),
            level_type,
            tree_node: todo!(),
            // parent: todo!(),
            // left: todo!(),
            // right: todo!(),
            // _pinned: PhantomPinned,
        }
    }
}