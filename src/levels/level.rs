

use std::{collections::LinkedList, cmp::Ordering, sync::Arc, rc::Rc, marker::PhantomData};

use orders::order::Order;

use crate::orders;

use super::indexing::Ref;

pub trait LevelOps<'a, R: Ref<'a>> {
    fn subtract_volumes(level: &mut Level<'a, R>, order: &Order<R>);
    fn unlink_order(level: &mut Level<'a, R>, order: &Order<R>);
    fn link_order(level: &mut Level<'a, R>, order: &Order<R>);
    fn add_volumes(level: &mut Level<'a, R>, order: &Order<R>);
}

// pub trait LevelMut<'a>: for<> DerefMut<Target = Level<'a, R>> {}

// impl<'a> LevelMut<'_> for &mut Level<'_, R> {}

// pub trait OrderRef<'a>: Deref<Target = Order<'a, R>> + AsRef<Order<'a, R>> {}

pub trait Mut<T> {}

impl<T> Mut<T> for & mut T {}

pub trait Count<T> {}

impl<T> Count<T> for Arc<T> {}

impl<T> Count<T> for Rc<T> {}


impl<'a, R: Ref<'a>> LevelOps<'a, R> for Level<'_, R> {
    fn subtract_volumes(level: &mut Level<'a, R>, order: &Order<R>) {
        level.total_volume -= order.leaves_quantity();
        level.hidden_volume -= order.hidden_quantity();
        level.visible_volume -= order.visible_quantity();
    }
    fn add_volumes(level: &mut Level<'a, R>, order: &Order<R>) {
        level.total_volume += order.leaves_quantity();
        level.hidden_volume += order.hidden_quantity();
        level.visible_volume += order.visible_quantity();
    }
    fn unlink_order(level: &mut Level<'a, R>, order: &Order<R>) {
        level.orders.pop_current(order); 
    }
    fn link_order(level: &mut Level<'a, R>, order: &Order<R>) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Level<'a, R> 
where
    R: Ref<'a>,
{
    pub price: u64,
    pub total_volume: u64,
    pub hidden_volume: u64,
    pub visible_volume: u64,
    pub(crate) orders: LinkedList<Order<'a, R>>,
    pub level_type: LevelType,
    pub(crate) _marker: PhantomData<&'a M>
}

impl<'a, R: Ref<'a>> From<Level<'a, R>> for LevelNode<'a, R> {
    fn from(level: Level<'a, R>) -> Self {
        LevelNode {
            level,
            parent: None,
            left: None,
            right: None,
            _marker: PhantomData,
        }
    }
}


impl<'a, R: Ref<'a>> Level<'a, R> {
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
            _marker: PhantomData, // Default value
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
    

    pub fn add_volumes(mut level: Level<R>, order: &Order<R>) {
        level.total_volume += order.leaves_quantity();
        level.hidden_volume += order.hidden_quantity();
        level.visible_volume += order.visible_quantity();
    }

    // pub fn link_order(&self, mut level: Level<R>, order: &Order<R>) {
    //     level.orders.pop_current(&Order<R>); // push_back for LinkedList
    //   
    // }
}


pub enum UpdateType {
    Add,
    Update,
    Delete,
}

pub struct LevelUpdate<'a, R> 
where
    R: Ref<'a>,
{
    pub(crate)update_type: UpdateType,
    pub(crate)update: Level<'a, R>, 
    pub(crate)top: bool,
}

pub trait PopCurrent <T, A>{
    fn pop_current(&mut self, value: &T) -> Option<R>;
}


impl<'a, R: Ref<'a>> From<Level<'a, R>> for *mut Level<'a, R> {
    fn from(value: Level<'a, R>) -> Self {
        Box::into_raw(Box::new(value))
    }
}


impl<T: PartialEq, A> PopCurrent<T, A> for LinkedList<T> {
    fn pop_current(&mut self, value: &T) -> Option<R> {
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


impl<'a, R: Ref<'a>> PartialEq for Level<'a, R> {
    fn eq(&self, other: &Self) -> bool {
        // Defer to Level's implementation of PartialEq
        self.price.eq(&other.price)
    }
}

impl<'a, R: Ref<'a>> Eq for Level<'a, R> {}

impl<'a, R: Ref<'a>> PartialOrd for Level<'a, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Defer to Level's implementation of PartialOrd
        self.price.partial_cmp(&other.price)
    }
}

impl<'a, R: Ref<'a>> Level<'a, R> {

    pub fn new_as_ptr(price: u64, level: Level<'a, R>, parent: Level<'a, R>) -> Self {
        Level {
            orders: todo!(),
            price,
            total_volume: todo!(),
            hidden_volume: todo!(),
            visible_volume: todo!(),
            level_type: todo!(),
            _marker: PhantomData,
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
            _marker: PhantomData,
            // parent: todo!(),
            // left: todo!(),
            // right: todo!(),
            // _pinned: PhantomPinned,
        }
    }
}