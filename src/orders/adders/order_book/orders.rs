use crate::{levels::{level::{UpdateType, LevelUpdate, Level}, indexing::Tree, }, order_book::order_book::OrderBook, orders::order::OrderNode};


impl OrderBook<'_> {

    pub fn add_order<T: Tree>(&self, order_node: &OrderNode) -> LevelUpdate {

        let mut update_type = UpdateType::Update;
        // Find the price level for the order
        let mut existing_level = if order_node.is_buy() {
            (*self.bids.borrow_mut()).get(&order_node.order.price)
        } else {
            (*self.asks.borrow_mut()).get(&order_node.order.price)
        };

        let binding: Level;
        if let None = existing_level {
            binding = self.add_level(order_node, Tree);
            existing_level = Some(&binding);
            update_type = UpdateType::Add;
        }

        let level: Level = Default::default();

        if let Some(level) = existing_level {
            let mut level = level.level;
            self.add_level_volumes(level, order_node);
            level.orders.push_back(order_node.clone());
            level.orders += 1;
            order_node.level = level;
        }

        LevelUpdate {
            update_type,
            update: Level { 
                level_type: level.level_type, 
                price: level.price, // Similarly for other fields
                total_volume: level.total_volume,
                hidden_volume: level.hidden_volume,
                visible_volume: level.visible_volume,
                orders: todo!(),
                tree_node: todo!(),
            },
            top: self.is_top_of_book(order_node),
        }
    }
}
