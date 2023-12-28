use std::{collections::LinkedList, ops::{Deref, DerefMut}, sync::{Arc, Mutex, MutexGuard}, fmt};

use crate::order::OrderNodeHandle;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelType {
    Bid,
    Ask,
}

#[derive(Default, Clone)]
pub struct LevelNodeHandle(Arc<Mutex<LevelNode>>);


impl LevelNodeHandle {
    pub fn new(node: LevelNode) -> Self {
        LevelNodeHandle(Arc::new(Mutex::new(node)))
    }
}

// Derive Debug implementation
impl fmt::Debug for LevelNodeHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LevelNodeHandle")
            .field(&self.0)
            .finish()
    }
}

impl PartialEq for LevelNodeHandle {
    fn eq(&self, other: &Self) -> bool {
        // Compare the memory addresses of the Arcs
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl LevelNodeHandle {
    // Method to lock and unwrap the mutex guard
    pub fn lock_unwrap(&self) -> MutexGuard<LevelNode> {
        self.0.lock().expect("Failed to lock LevelNode mutex")
    }
}

#[derive(Debug, Default)]
pub struct LevelNode {
    pub(crate) price: u64,
    pub(crate) level: Level,
    pub(crate) order_list: LinkedList<OrderNodeHandle>, 
    pub(crate) orders: LinkedList<OrderNodeHandle>,
    pub left: LevelNodeHandle,
    pub right: LevelNodeHandle,
    pub parent: LevelNodeHandle,
    pub(crate) level_type: Option<LevelType>,
}

impl LevelNode {
    pub fn new(price: u64, level: Level, parent: LevelNodeHandle) -> Self {
        LevelNode {
            price,
            order_list: LinkedList::new(),
            left: LevelNodeHandle::default(),
            right: LevelNodeHandle::default(),
            parent,
            orders: todo!(),
            level_type: None,
            level: level,
        }
    }
    pub fn is_bid(&self) -> bool {
        self.level.level_type == Some(LevelType::Bid)
    }
}

impl PartialEq for LevelNode {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Deref for LevelNode {
    type Target = LinkedList<OrderNodeHandle>;

    fn deref(&self) -> &Self::Target {
        &self.orders
    }
}

impl DerefMut for LevelNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.orders
    }
}


impl Eq for LevelNode {}

// Assuming the following enums and structs are defined as per your requirements
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

#[derive(PartialEq, Debug, Default, Clone)]
pub struct OrderList<T> {
    pub list: LinkedList<T>,
}

impl<T: PartialEq> OrderList<T> {
    pub fn pop_current(&mut self, value: &T) -> Option<T> {
        let mut new_list = LinkedList::new();
        let mut removed_item = None;

        while let Some(item) = self.list.pop_front() {
            if item == *value && removed_item.is_none() {
                removed_item = Some(item);
            } else {
                new_list.push_back(item);
            }
        }

        self.list = new_list;
        removed_item
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Level {
    pub level_type: Option<LevelType>,
    pub price: u64,
    pub total_volume: u64,
    pub hidden_volume: u64,
    pub visible_volume: u64,
    pub order_list: OrderList<OrderNodeHandle>,
    pub orders: u64
}

impl Level {
    fn new(level_type: LevelType, price: u64) -> Self {
        Level {
            level_type: Some(level_type),
            price,
            total_volume: 0,
            hidden_volume: 0,
            visible_volume: 0,
            order_list: todo!(),
            orders: todo!(),
        }
    }

    pub fn is_bid(&self) -> bool {
        self.level_type == Some(LevelType::Bid)
    }

    pub fn is_ask(&self) -> bool {
        self.level_type == Some(LevelType::Ask)
    }
}


