
use crate::{orders::order::OrderNode, order_book::order_book::BookOps, levels::level::PopCurrent, market_executors::executor::Execution, references::Convertible};



pub fn delete_trailing_stop_order<E: for<'a> Execution<'a>, B: for<'a> BookOps<'a>, C: Convertible<B>>(order_book: C, order_node: &OrderNode) -> Result<(), &'static str> {

    // remove panicking behavior from code
    let mut level_node = order_node.level_node.expect("level node not retrieved from order node");
    let level = (*level_node.borrow()).level;
    // Update the price level volume
    E::subtract_level_volumes(level, order_node);

    // Unlink the empty order from the orders list of the price level
    level.orders.pop_current(&order_node); // Assuming each order has a unique identifier

    // Delete the empty price level
    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        order_book.delete_trailing_stop_level(order_node);
    };
    Ok(())
}

pub fn reduce_trailing_stop_order<E: for<'a> Execution<'a>>(order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) {
    // Assuming we have a way to get a mutable reference to an order and its level.
    // Update the price level volume
    // remove panicking behavior from code
    let mut level_node = order_node.level_node.expect("level node not retrieved from order node");
    let level = (*level_node.borrow()).level;
    // Update the price level volume
    E::subtract_level_volumes(level, order_node);
    // Unlink the empty order from the orders list of the price level
    if order_node.leaves_quantity == 0 {
        E::unlink_order(level, order_node);
    }
    // Delete the empty price level
    if level.total_volume == 0 {
        order_node.level_node = None
    }
}

