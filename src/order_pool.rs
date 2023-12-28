

use crate::{order::OrderHandle, OrderNodeHandle};

#[derive(Clone)]
pub struct OrderPool {
    pool: Vec<OrderNodeHandle>,
    next_free: u64,
}

impl OrderPool {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: u64) -> Self {
        let mut pool = Vec::with_capacity(capacity.try_into().unwrap());
        Self { pool, next_free: 0 }
    }

    pub fn create(&mut self, order_handle: &OrderHandle) -> Option<&OrderNodeHandle> {
        if self.next_free == self.pool.len() as u64 {
            self.pool.push(OrderNodeHandle::default());
            self.next_free += 1;
            
            // Use `as_mut` instead of `as_ref` to get a mutable reference
            self.pool.last().and_then(|node| Some(node))
        } else {
            if let Some(node) = self.allocate() {
                node.clone().lock_unwrap().order_handle = order_handle.clone(); // Again, assuming Order implements Copy
                Some(node)
            } else {
                None
            }
        }
    }

    pub fn allocate(&mut self) -> Option<&OrderNodeHandle> {
        if self.next_free < self.pool.len().try_into().unwrap() {
            let node_option = self.pool.get(self.next_free as usize);
            self.next_free += 1; // Increment next_free for the next call

            // Since we checked that next_free is within bounds, node_option should not be None
            node_option.and_then(|node| Some(node))
        } else {
            None // Return None if next_free is out of bounds
        }
    }

    pub fn deallocate(&mut self, node: OrderNodeHandle) {
        let id = node.clone().lock_unwrap().id;
        self.pool[id as usize] = node;
        if id < self.next_free {
            self.next_free = id;
        }
    }

    pub fn release(&mut self, mut node: OrderNodeHandle) {
        // Reset the fields of the node to a default state
        node.clone().lock_unwrap().order_handle = OrderHandle::default(); // Assuming Order implements Default

        // Add the node back to the pool
        let id = node.clone().lock_unwrap().id as u64;
        self.pool[id as usize] = node;

        // Update the next_free pointer if necessary
        if id < self.next_free {
            self.next_free = id;
        }
    }
}
