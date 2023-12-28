
use crate::{LevelNode, LevelType, LevelNodeHandle};


#[derive(Debug, Default)]
pub struct LevelPool {
    allocated: Vec<LevelNodeHandle>,
    free: Vec<u64>,
}

impl<'a> LevelPool {
    pub fn create(&mut self, level_type: LevelType, price: u64) -> LevelNodeHandle {
        // Define a helper closure that takes LevelNodeHandle and checks if it matches the given level_type and price
        let matches_level = |level_handle: &LevelNodeHandle, level_type: LevelType, price: u64| -> bool {
            let ln = level_handle.lock_unwrap();
            ln.level_type == Some(level_type) && ln.price == price
        };

        // Try to find an existing LevelNodeHandle
        if let Some(level_handle) = self.allocated.iter().find(|&ln_handle| matches_level(ln_handle, level_type.clone(), price)) {
            return level_handle.clone(); // Clone the Arc
        }

        // If not found, create a new LevelNode and wrap it in a LevelNodeHandle
        let new_level_node = LevelNodeHandle::new(LevelNode::default());

        // Add the new LevelNodeHandle to the allocated vector
        self.allocated.push(new_level_node.clone());

        new_level_node
    }

    pub fn release(&mut self, price: u64) -> LevelNodeHandle {
        // Logic for releasing a level node
        if price < self.allocated.len() as u64 {
            self.free.push(price);
        }
        LevelNodeHandle::new(LevelNode::default())
    }
}

