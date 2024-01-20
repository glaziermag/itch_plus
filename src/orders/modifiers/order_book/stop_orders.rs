use crate::{levels::level::LevelNode, orders::order::OrderNode, order_book::order_book::OrderBook};


pub trait TrailingStopOrders {
    fn reduce_stop_order(&self, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64);
    fn delete_stop_order(&self, order_node: &OrderNode);
}


impl TrailingStopOrders for OrderBook<'_> {

  fn reduce_stop_order(&self, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) {
        
        // Find the price level for the order
        let mut level = order_node.level_node.level;

        // Update the price level volume
        level.total_volume -= quantity;
        level.hidden_volume -= hidden;
        level.visible_volume -= visible;
        // Unlink the empty order from the orders list of the price level
        if order_node.leaves_quantity == 0 {
            // Assuming pop_current is a function that removes an order based on Some criteria and returns an Option<order /* OrderNode */>
            level.orders.pop_current(&order_node);
            level.orders -= 1;
        }
        // Delete the empty price level
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_stop_level(order_node);
        };
    }

    fn delete_stop_order(&self, order_node: &OrderNode) {
        
        // Update the price level volume
        let mut level = order_node.level_node.level;
        level.total_volume -= order_node.order.leaves_quantity;
        level.hidden_volume -= order_node.order.hidden_quantity();
        level.visible_volume -= order_node.order.visible_quantity();

        // Unlink the empty order from the orders list of the price level
        let _ = level.orders.pop_current(&order_node).ok_or("Failed to remove order from order list");
        level.orders -= 1;

        // Delete the empty price level
        if level.total_volume == 0 {
            self.delete_stop_level(order_node);
        }
    }
}