

use crate::{orders::{order::{Order, ErrorCode}, orders::OrderOps}, market_executors::executor::Execution, market_handler::Handler, order_book::order_book::BookOps};


pub fn add_limit_order<E: for<'a> Execution<'a>, H: Handler, B: for<'a> BookOps<'a>, O: OrderOps>(orders: O, order: Order, matching: bool, order_books: B, recursive: bool) -> Result<(), ErrorCode> {
        
    let mut order_book = order_books.get_order_book(order.symbol_id);
    let order_node = E::get_order_node(order.symbol_id);

    H::on_add_order(&order);

    if matching && !recursive {
        E::match_limit(order_book, order);
    }

    if (order.leaves_quantity > 0) && !order.is_ioc() && !order.is_fok() {
    // let order_node = order_node.new(&order);
        if O::orders.insert(order_node.new(&order).id, order_node.new(&order)).is_some() {
            // Order duplicate
            H::on_delete_duplicate_order(order_node);
            // order_pool.release(order_node.new(&order));
        } else {
            // Update level with the new order
            E::update_level(order_book.add_order(order_node.new(&order)));
        }
    } else {
        H::on_delete_unmatched_order(order);
    }

    if matching && !recursive {
        E::match_order_book(order_book);
    }

    order_book.reset_matching_price();

    Ok(())
}
