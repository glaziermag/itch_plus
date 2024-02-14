use std::{collections::HashMap, ops::{Deref, DerefMut}};

use super::order::{ErrorCode, Order};

pub trait OrderOps
{
    fn insert_order(&mut self, id: &u64, order: Order) -> Option<Order>;
    fn remove_order(&mut self, id: &u64) -> Option<Order>;
    fn get_order(&self, id: u64) -> Result<&Order, ErrorCode>;
}

impl OrderOps for Orders 
{
    fn insert_order(&mut self, id: &u64, order: Order) -> Option<Order> {
        self.insert(*id, order)
    }

    fn remove_order(&mut self, id: &u64) -> Option<Order> {
        self.remove(&id)
    }

    fn get_order(&self, id: u64) -> Result<&Order, ErrorCode> {
        self.orders.get(&id).ok_or(ErrorCode::OrderNotFound)
    }
}

#[derive(Default)]
pub struct Orders
{
    orders: HashMap<u64, Order>
}


impl Deref for Orders 
{
    type Target = HashMap<u64, Order>;

    fn deref(&self) -> &Self::Target {
        &self.orders
    }
}

impl DerefMut for Orders 
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.orders
    }
}

