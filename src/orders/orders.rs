use std::collections::HashMap;

use super::order::Order;


pub trait Orders {
    fn insert_order(&mut self, id: u64, order: Order);
    fn remove_order(&mut self, id: u64) -> Option<Order>;
}

impl Orders for HashMap<u64, Order> {
    fn insert_order(&mut self, id: u64, order: Order) {
        self.insert(id, order);
    }

    fn remove_order(&mut self, id: u64) -> Option<Order> {
        self.remove(&id)
    }
}
