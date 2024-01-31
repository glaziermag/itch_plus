use crate::{levels::level::{Level}, orders::order::OrderNode, order_book::order_book::OrderBook};

impl OrderBook<'_> {

    pub fn add_stop_order(&self, order_node: &OrderNode) {
        // Find the price level for the order
        let level = if order_node.is_buy() {
            (*self.buy_stop.borrow_mut()).get(&order_node.stop_price)
        } else {
            (*self.sell_stop.borrow_mut()).get(&order_node.stop_price)
        };

        let binding: Level;
        let level = match level {
            level => level,
            None => {
                binding = self.add_stop_level(order_node);
                Some(&binding)
            },
        };

        let level: Level;
        if let Some(level) = level {
            level = level.level;
            self.add_level_volumes(level, order_node);
            // Link the new order to the orders list of the price level
            level.orders.list.push_back(order_node); 
            level.orders += 1;
            order_node.level = level
        } else {
            level = level.level;
            order_node.level = level
        }
    }
    
}