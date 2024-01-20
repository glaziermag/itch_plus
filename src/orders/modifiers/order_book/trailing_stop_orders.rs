
use crate::{orders::order::OrderNode, order_book::order_book::OrderBook};

pub trait TrailingStopOrders {
    fn reduce_trailing_stop_order(&self, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64);
    fn delete_trailing_stop_order(&self, order_node: &OrderNode) -> Result<(), &'static str>;
}

impl TrailingStopOrders for OrderBook<'_> {

    fn delete_trailing_stop_order(&self, order_node: &OrderNode) -> Result<(), &'static str> {

        let mut level = order_node.level_node.level;
        // Update the price level volume
        self.subtract_level_volumes(level, order_node);

        // Unlink the empty order from the orders list of the price level
        level.orders.pop_current(&order_node); // Assuming each order has a unique identifier
        level.orders -= 1; // Adjusting the orders count

        // Delete the empty price level
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_trailing_stop_level(order_node);
        };
        Ok(())
    }

    fn reduce_trailing_stop_order(&self, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) {
        // Assuming we have a way to get a mutable reference to an order and its level.
        // Update the price level volume
        let mut level = order_node.level_node.level;
        // Update the price level volume
        self.subtract_level_volumes(level, order_node);
        // Unlink the empty order from the orders list of the price level
        if order_node.leaves_quantity == 0 {
            self.unlink_order(level, order_node);
        }
        // Delete the empty price level
        if level.total_volume == 0 {
            order_node.level_node = self.delete_trailing_stop_level(order_node)
        }
    }
}
