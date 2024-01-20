use crate::{levels::level::{UpdateType, LevelUpdate, LevelNode, Level}, order_book::order_book::OrderBook, orders::order::OrderNode};


trait Orders {
    fn delete_order(&self, order_node: &OrderNode) -> LevelUpdate;
    fn reduce_order(&self, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate;
}

impl Orders for OrderBook<'_> {

    fn reduce_order(&self, mut order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate {

        let mut update_type = UpdateType::Update;
        let mut level_update: LevelUpdate;

        let mut level = order_node.level_node.level;
        level.total_volume -= quantity;
        level.hidden_volume -= hidden;
        level.visible_volume -= visible;

        if order_node.leaves_quantity == 0 {
            //self.unlink_order(level, order_node)
            level.orders.pop_current(&order_node);
            level.orders -= 1
        }

        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_level(order_node);
            order_node.level_node = self.delete_level(order_node);
            update_type = UpdateType::Delete;
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

    fn delete_order(&self, order_node: &OrderNode) -> LevelUpdate {

        let mut level = order_node.level_node.level;
        
        // Update the price level volume
        self.subtract_level_volumes(level, order_node);

        // Unlink the empty order from the orders list of the price level
        self.unlink_order(level, order_node);

        let mut update_type = UpdateType::Update;
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            level = self.delete_level(order_node).level;
            update_type = UpdateType::Delete;
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
