use crate::{levels::level::{UpdateType, LevelUpdate, LevelNode, Level}, order_book::order_book::OrderBook, orders::order::OrderNode};


trait Orders {
    fn add_order(&self, order_node: &OrderNode) -> LevelUpdate; 
}

impl Orders for OrderBook<'_> {

    fn add_order(&self, order_node: &OrderNode) -> LevelUpdate {

        let mut update_type = UpdateType::Update;
        // Find the price level for the order
        let mut existing_level_node = if order_node.is_buy() {
            self.bids.get(&order_node.order.price)
        } else {
            self.asks.get(&order_node.order.price)
        };

        let binding: LevelNode;
        if let None = existing_level_node {
            binding = self.add_level(order_node);
            existing_level_node = Some(&binding);
            update_type = UpdateType::Add;
        }

        let level: Level = Default::default();

        if let Some(level_node) = existing_level_node {
            let mut level = level_node.level;
            self.add_level_volumes(level, order_node);
            level_node.orders.push_back(order_node.clone());
            level_node.level.orders += 1;
            order_node.level_node = level_node;
        }

        LevelUpdate {
            update_type,
            update: Level { 
                level_type: level.level_type, 
                price: level.price, // Similarly for other fields
                total_volume: level.total_volume,
                hidden_volume: level.hidden_volume,
                visible_volume: level.visible_volume,
                orders: level.orders,
                orders: level.orders,
            },
            top: self.is_top_of_book(order_node),
        }
    }
}
