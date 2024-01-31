use crate::{orders::order::OrderNode, order_book::order_book::OrderBook, levels::level::{Level}};

impl OrderBook<'_> {

    pub fn add_trailing_stop_order(&self, order_node: &OrderNode) {

        let level = if order_node.is_buy() {
            (*self.trailing_buy_stop.borrow_mut()).get(&order_node.stop_price)
                .or_else(|| {
                    let mut binding: Level = self.add_trailing_stop_level(order_node);
                    Some(&binding)
                })// Clones the Arc, not the Level
        } else {
            (*self.trailing_sell_stop.borrow_mut()).get(&order_node.stop_price)
                .or_else(|| {
                    let mut binding: Level = self.add_trailing_stop_level(order_node);
                    Some(&binding)
                }) // Clones the Arc, not the Level
        };

        let node = level;

        let mut level = &node.level;

        // Update the price level volume
        self.add_level_volumes(*level, order_node);

        // Link the new order to the orders list of the price level
        self.link_order(*level, order_node);

        // Unlink the empty order from the orders list of the price level
        level.orders.list.push_back(order_node);
        level.orders += 1;

        order_node.level = level;
    }
}