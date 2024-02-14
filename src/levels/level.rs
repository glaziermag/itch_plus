
use std::{collections::LinkedList, cmp::Ordering};

use orders::order::Order;
use crate::orders;

use super::indexing::{LevelNode};


pub trait LevelOps 
{
    fn subtract_volumes(&mut self, order: &Order);
    fn unlink_order(&mut self, order: &Order);
    fn link_order(&mut self, order: &Order);
    fn add_volumes(&mut self, order: &Order);
}

impl LevelOps for Level  
{
    fn subtract_volumes(&mut self, order: &Order) {
        self.total_volume -= order.leaves_quantity();
        self.hidden_volume -= order.hidden_quantity();
        self.visible_volume -= order.visible_quantity();
    }
    fn add_volumes(&mut self, order: &Order) {
        self.total_volume += order.leaves_quantity();
        self.hidden_volume += order.hidden_quantity();
        self.visible_volume += order.visible_quantity();
    }
    fn unlink_order(&mut self, order: &Order) {
        self.orders.pop_current(order); 
    }
    fn link_order(&mut self, order: &Order) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Level {
    pub price: u64,
    pub total_volume: u64,
    pub hidden_volume: u64,
    pub visible_volume: u64,
    pub(crate) orders: LinkedList<Order>,
    pub level_type: LevelType,
}

impl From<Level> for LevelNode {
    fn from(level: Level) -> Self {
        LevelNode {
            level,
            parent: None,
            left: None,
            right: None,
        }
    }
}

impl Level {
    /// Creates a new `Level` instance.
    /// 
    /// The method returns `Pin<Box<Self>>` because `Level` instances might contain
    /// self-referential data (e.g., parent, left, Aight raw pointers). `Pin` guarantees
    /// that the `Level` instance will not be moved in memory, which is crucial for 
    /// maintaining the validity of self-referential pointers.

    pub fn with_price(level_type: LevelType, price: u64) -> Self {
        Level {
            price,
            total_volume: 0,  // Default value
            hidden_volume: 0, // Default value
            visible_volume: 0, // Default value
            orders: LinkedList::new(), // Initialize with an empty LinkedList
            level_type,// Default value
            // parent: None,  // No parent initially
            // left: None,    // No left child initially
            // right: None,   // No right child initially
            // _pinned: PhantomPinned,
        }
    }

    pub fn is_bid(&self) -> bool {
        self.level_type == LevelType::Bid
    }

    pub fn is_ask(&self) -> bool {
        self.level_type == LevelType::Ask
    }
    

    pub fn add_volumes(&mut self, order: &Order) {
        self.total_volume += order.leaves_quantity();
        self.hidden_volume += order.hidden_quantity();
        self.visible_volume += order.visible_quantity();
    }

    // pub fn link_order(&self, mut level: Level, order: &Order) {
    //     level.orders.pop_current(&Order); // push_back for LinkedList
    //   
    // }
}


pub enum UpdateType {
    Add,
    Update,
    Delete,
}

pub struct LevelUpdate {
    pub(crate)update_type: UpdateType,
    pub(crate)update: Level, 
    pub(crate)top: bool,
}

pub trait PopCurrent<T> {
    fn pop_current(&mut self, value: &T) -> Option<T>;
}


// impl From<Level> for LevelNode {
//     fn from(value: Level) -> Self {
//         Box::into_raw(Box::new(value))
//     }
// }


impl<T: PartialEq> PopCurrent<T> for LinkedList<T> {
    fn pop_current(&mut self, value: &T) -> Option<T> {
        let mut new_list = LinkedList::new();
        let mut removed_item: Option<T> = None;

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


impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        // Defer to Level's implementation of PartialEq
        self.price.eq(&other.price)
    }
}

impl Eq for Level {}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Defer to Level's implementation of PartialOrd
        self.price.partial_cmp(&other.price)
    }
}

impl Level {

    pub fn new_as_ptr(price: u64, level: Level, parent: Level) -> Self {
        Level {
            orders: todo!(),
            price,
            total_volume: todo!(),
            hidden_volume: todo!(),
            visible_volume: todo!(),
            level_type: todo!(),
            // parent: todo!(),
            // left: todo!(),
            // right: todo!(),
            // _pinned: PhantomPinned,
        }
    }

    pub fn new_from_order(price: u64, level_type: LevelType) -> Self {
        Level {
            orders: todo!(),
            price,
            total_volume: todo!(),
            hidden_volume: todo!(),
            visible_volume: todo!(),
            level_type,
            // parent: todo!(),
            // left: todo!(),
            // right: todo!(),
            // _pinned: PhantomPinned,
        }
    }
}