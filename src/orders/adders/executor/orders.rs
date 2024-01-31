

use crate::{market_executors::{executor::{MarketExecutor, Execution}}, market_handler::{MarketHandler, Handler}, orders::{order::{ErrorCode, Order, OrderType}, orders::{Orders, OrderOps}}, order_book::order_book::BookOps};


pub fn add_order<E: for<'a> Execution<'a>, B: for<'a> BookOps<'a>, H: Handler, O: OrderOps>(orders: O, order_books: B, order: Order, matching: bool, recursive: bool, market_handler: H) -> Result<(), ErrorCode> {
    order.validate();
    // let some_condition = true;
    // if some_condition {
    //     matching = true;
    //     recursive = false;
    // }

    match order.order_type {
        OrderType::Buy => {
            // Handle Buy orders
            todo!()
        },
        OrderType::Market => {
            E::add_market_order(order_books, order, matching, recursive, market_handler)
        },
        OrderType::Limit => {
            E::add_limit_order(order, matching, order_books, recursive, market_handler)
        },
        OrderType::Stop | OrderType::TrailingStop => {
            E::add_stop_order(orders, order_books, order, matching, recursive, market_handler)
        },
        OrderType::StopLimit | OrderType::TrailingStopLimit => {
            E::add_stop_limit_order(order_books, orders, market_handler, order, matching, recursive)
        },
        _ => Err(ErrorCode::OrderTypeInvalid),
    }
}

pub fn add_market_order<E: for<'a> Execution<'a>, H: Handler, B: for<'a> BookOps<'a>>(order_books: B, order: Order, matching: bool, recursive: bool, market_handler: H) -> Result<(), ErrorCode> {

    let mut order_book = order_books.get_order_book(order.symbol_id);

    // let some_condition = true;
    // if some_condition {
    //     matching = true;
    //     recursive = false;
    // }

    H::on_add_order(order);

    if matching && !recursive {
        E::match_market(order_book, order);
    }

    H::on_delete_order(order);

    if matching && !recursive {
        E::match_order_book(order_book, market_handler); // Assuming match_order also returns a Result
    }
    
    let mut order_book = order_book.reset_matching_price();

    Ok(())
}
