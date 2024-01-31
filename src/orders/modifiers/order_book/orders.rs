use crate::{levels::level::{UpdateType, LevelUpdate, Level, PopCurrent}, order_book::order_book::{BookOps}, orders::order::OrderNode};



pub fn reduce_order<B: for<'a> BookOps<'a>>(order_book: C, mut order_node: &'a OrderNode<'a>, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate<'a> {

    let mut update_type = UpdateType::Update;
    let mut level_update: LevelUpdate;

    // remove panicking behavior from code
    let mut level_node = (*order_node.level_node.expect("level node not retrieved from order node").borrow_mut());
    let mut level = level_node.level;
    level.total_volume -= quantity;
    level.hidden_volume -= hidden;
    level.visible_volume -= visible;

    if order_node.leaves_quantity == 0 {
        //self.unlink_order(level, order_node)
        level.orders.pop_current(&order_node);
    }

    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_level(order_node);
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
            orders: todo!(),
            tree_node: todo!(),
        },
        top: B::is_top_of_book(order_node),
    }
}

pub fn delete_order<B: for<'a> BookOps<'a>>(order_book: C, order_node: &OrderNode) -> LevelUpdate {

    // remove panicking behavior from code
    let mut level = order_node.level_node.expect("level node not retrieved from order node");
    
    // Update the price level volume
    B::subtract_level_volumes(level, order_node);

    // Unlink the empty order from the orders list of the price level
    B::unlink_order(level, order_node);

    let mut update_type = UpdateType::Update;
    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        level = B::delete_level(order_node).level;
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
            orders: todo!(),
            tree_node: todo!(),
        },
        top: B::is_top_of_book(order_node),
    }
}
