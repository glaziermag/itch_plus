use crate::{levels::level::PopCurrent, orders::order::OrderNode, order_book::order_book::OrderBook};

impl OrderBook<'_> {

  pub fn reduce_stop_order(&self, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) {
        
        // Find the price level for the order
        // remove panicking behavior from code
        let mut level_node = order_node.level_node.expect("level node not retrieved from order node");
        let mut level_borrow = level_node.borrow().level;

        // Update the price level volume
        level_borrow.total_volume -= quantity;
        level_borrow.hidden_volume -= hidden;
        level_borrow.visible_volume -= visible;
        // Unlink the empty order from the orders list of the price level
        if order_node.leaves_quantity == 0 {
            // Assuming pop_current is a function that removes an order based on Some criteria and returns an Option<order /* OrderNode */>
            level_borrow.orders.pop_current(&order_node);
           // level_borrow.orders -= 1;
        }
        // Delete the empty price level
        if level_borrow.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_stop_level(order_node);
        };
    }

    pub fn delete_stop_order(&self, order_node: &OrderNode) {
        
        // Update the price level volume
        // Find the price level for the order
        // remove panicking behavior from code
        let mut level_node = order_node.level_node.expect("level node not retrieved from order node");
        let mut level_borrow = level_node.borrow().level;

        level_borrow.total_volume -= order_node.order.leaves_quantity();
        level_borrow.hidden_volume -= order_node.order.hidden_quantity();
        level_borrow.visible_volume -= order_node.order.visible_quantity();

        // Unlink the empty order from the orders list of the price level
        level_borrow.orders.pop_current(&order_node);

        // Delete the empty price level
        if level_borrow.total_volume == 0 {
            self.delete_stop_level(order_node);
        }
    }
}