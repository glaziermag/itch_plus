use crate::{levels::level::{LevelNode, Level}, orders::order::OrderNode, order_book::order_book::OrderBook};


pub trait StopOrders {
    fn add_stop_order(&self, order_node: &OrderNode);
}


impl StopOrders for OrderBook<'_> {

    fn add_stop_order(&self, order_node: &OrderNode) {
        // Find the price level for the order
        let level_node = if order_node.is_buy() {
            self.buy_stop.get(&order_node.stop_price)
        } else {
            self.sell_stop.get(&order_node.stop_price)
        };

        let binding: LevelNode;
        let level_node = match level_node {
            level_node => level_node,
            None => {
                binding = self.add_stop_level(order_node);
                Some(&binding)
            },
        };

        let level: Level;
        if let Some(level_node) = level_node {
            level = level_node.level;
            self.add_level_volumes(level, order_node);
            // Link the new order to the orders list of the price level
            level.orders.list.push_back(order_node); 
            level.orders += 1;
            order_node.level_node.level = level;
        } else {
            level = level_node.level;
            order_node.level_node.level = level;
        }
    }
    
}