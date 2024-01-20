use crate::{levels::level::{LevelNode, LevelType}, orders::order::OrderNode};



pub trait Factory {
    type Item;

    fn create(level_type: LevelType, order_node: &OrderNode) -> Self::Item;
    fn with_price(level_type: LevelType, price: u64) -> Self::Item;
}

impl Factory for LevelNode {
    type Item = LevelNode;

    fn create(level_type: LevelType, order_node: &OrderNode) -> Self::Item {
        LevelNode::create(level_type, order_node)
    }

    fn with_price(level_type: LevelType, price: u64) -> Self::Item {
        // Implementation for creating LevelNode with a specific price
        LevelNode::with_price(level_type, price)
    }
}
