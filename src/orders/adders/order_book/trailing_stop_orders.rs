use crate::{orders::order::OrderNode, order_book::order_book::OrderBook, levels::level::LevelNode};

pub trait TrailingStopOrders {
    fn add_trailing_stop_order(&self, order_node: &OrderNode);
}


impl TrailingStopOrders for OrderBook<'_> {

    fn add_trailing_stop_order(&self, order_node: &OrderNode) {

        let level_node = if order_node.is_buy() {
            let mut binding: LevelNode = Default::default();
            self.trailing_buy_stop.get(&order_node.stop_price)
                .or_else(|| {
                    binding = self.add_trailing_stop_level(order_node);
                    Some(&binding)
                })// Clones the Arc, not the LevelNode
        } else {
            let mut binding: LevelNode = Default::default();
            self.trailing_sell_stop.get(&order_node.stop_price)
                .or_else(|| {
                    binding = self.add_trailing_stop_level(order_node);
                    Some(&binding)
                }) // Clones the Arc, not the LevelNode
        };

        let node = level_node;

        let mut level = &node.level;

        // Update the price level volume
        self.add_level_volumes(*level, order_node);

        // Link the new order to the orders list of the price level
        self.link_order(*level, order_node);

        // Unlink the empty order from the orders list of the price level
        level.orders.list.push_back(order_node);
        level.orders += 1;

        order_node.level_node.level = level;
    }
}