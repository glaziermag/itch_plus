use std::{collections::LinkedList, ops::{Deref, DerefMut}, rc::{Weak, Rc}, cell::RefCell, cmp::Ordering};

use crate::{orders::order::OrderNode, order_book::order_book::OrderBook};

use super::node::{Node, TreeNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelType {
    Bid,
    Ask,
}

#[derive(Debug, Default)]
pub struct LevelNode
where
{
    pub(crate) level: Level,
    pub(crate) orders: LinkedList<OrderNode>,
    pub(crate) node: TreeNode
}

impl PartialEq for LevelNode 
{
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl  Eq for LevelNode  {}

impl PartialOrd for LevelNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl LevelNode {
    pub fn new(price: u64, level: Level, parent: LevelNode) -> Self {
        LevelNode {
            orders: LinkedList::new(),
            orders: todo!(),
            level: level,
            node: todo!(),
        }
    }

    pub fn new_as_ptr(price: u64, level: Level, parent: &Rc<RefCell<LevelNode>>) -> Self {
        LevelNode {
            orders: LinkedList::new(),
            orders: todo!(),
            level: level,
            node: todo!(),
        }
    }

    pub fn new_from_order_node(price: u64, level_type: LevelType) -> Self {
        LevelNode {
            level: Level::default(), // Assuming Level has a default or provide an appropriate value
            orders: LinkedList::new(),
            node: todo!(), 
        }
    }

    pub fn is_bid(&self) -> bool {
        self.level.level_type == Some(LevelType::Bid)
    }
}

impl  Deref for LevelNode {
    type Target = LinkedList<OrderNode>;

    fn deref(&self) -> &Self::Target {
        &self.orders
    }
}

impl  DerefMut for LevelNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.orders
    }
}

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

impl<T> OrderList<T> {
    pub fn pop_current(&mut self, value: &OrderBook) -> Option<T> {
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
    pub orders: OrderList<OrderNode>,
}

impl Level {
    fn new(level_type: LevelType, price: u64) -> Self {
        Level {
            level_type: Some(level_type),
            price,
            total_volume: 0,
            hidden_volume: 0,
            visible_volume: 0,
            orders: todo!(),
            orders: todo!(),
        }
    }

    pub fn is_bid(&self) -> bool {
        self.level_type == Some(LevelType::Bid)
    }

    pub fn is_ask(&self) -> bool {
        self.level_type == Some(LevelType::Ask)
    }
    
    fn subtract_level_volumes(&self, mut level: Level, order_node: &OrderNode) {
        level.total_volume -= order_node.order.leaves_quantity();
        level.hidden_volume -= order_node.order.hidden_quantity();
        level.visible_volume -= order_node.order.visible_quantity();
    }

    fn add_level_volumes(&self, mut level: Level, order_node: &OrderNode) {
        level.total_volume += order_node.order.leaves_quantity;
        level.hidden_volume += order_node.order.hidden_quantity();
        level.visible_volume += order_node.order.visible_quantity();
    }

    pub fn link_order(&self, mut level: Level, order_node: &OrderNode) {
        level.orders.pop_current(&order_node); // push_back for LinkedList
        level.orders += 1;
    }

    pub fn unlink_order(&self, mut level: Level, order_node: &OrderNode) {
        level.orders.pop_current(&order_node); 
        level.orders -= 1;
    }
}


