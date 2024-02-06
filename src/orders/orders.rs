use std::{collections::HashMap, ops::DerefMut};


use crate::levels::indexing::{MutableBook};

use super::order::Order;

pub trait OrderOps<'a, B, R> 
where 
    B: MutableBook<'a>,
    R: Ref<'a>,
{
    fn insert_order(orders: Orders<'a, B>, id: &'a u64, order: Order<'a, R>) -> Option<Order<'a, R>>;
    fn remove_order(orders: Orders<'a, B>, id: &'a u64) -> Option<Order<'a, R>> ;
}

impl<'a, B> OrderOps<'a, B> for Orders<'_,  B> {
    fn insert_order(mut orders: Orders<'a, B>, id: &'a u64, order: Order<'a, R>) -> Option<Order<'a, R>> {
        orders.insert(*id, order)
    }

    fn remove_order(mut orders: Orders<'a, B>, id: &'a u64) -> Option<Order<'a, R>> {
        orders.remove(&id)
    }
}

pub struct Orders<'a, B> {
    orders: HashMap<u64, Order<'a, R>>
}


impl<'a, B> Deref for Orders<'a, B> {
    type Target = HashMap<u64, Order<'a, R>>;

    fn deref(&self) -> &Self::Target {
        &self.orders
    }
}

impl<'a, B> DerefMut for Orders<'a, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.orders
    }
}

