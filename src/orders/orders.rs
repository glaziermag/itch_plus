use std::{collections::HashMap, hash::Hash, ops::{Deref, DerefMut}};

use super::order::Order;

pub trait OrderOps {

}

impl OrderOps for Orders {
    
}

pub struct Orders {
    orders: HashMap<u64, Order>
}


impl<'a> Deref for Orders {
    type Target = HashMap<u64, Order>;

    fn deref(&self) -> &Self::Target {
        &self.orders
    }
}

impl<'a> DerefMut for Orders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.orders
    }
}

impl Orders {
    pub fn insert_order(&mut self, id: u64, order: Order) {
        self.insert(id, order);
    }

    pub fn remove_order(&mut self, id: u64) -> Option<Order> {
        self.remove(&id)
    }
}
