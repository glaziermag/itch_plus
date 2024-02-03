use std::{collections::HashMap, ops::{Deref, DerefMut}};

use super::order::{Order, ErrorCode};

pub trait OrderOps {
    fn insert_order<'a>(orders: Orders<'a>, id: &'a u64, order: Order<'a>) -> Option<Order<'a>>;
    fn remove_order<'a>(orders: Orders<'a>, id: &'a u64) -> Option<Order<'a>> ;
}

impl OrderOps for Orders<'_> {
    fn insert_order<'a>(mut orders: Orders<'a>, id: &'a u64, order: Order<'a>) -> Option<Order<'a>> {
        orders.insert(*id, order)
    }

    fn remove_order<'a>(mut orders: Orders<'a>, id: &'a u64) -> Option<Order<'a>> {
        orders.remove(&id)
    }
}

pub struct Orders<'a> {
    orders: HashMap<u64, Order<'a>>
}


impl<'a> Deref for Orders<'a> {
    type Target = HashMap<u64, Order<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.orders
    }
}

impl<'a> DerefMut for Orders<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.orders
    }
}

