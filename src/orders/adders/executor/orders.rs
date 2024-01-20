

use crate::{market_executors::{executor::{MarketExecutor, Executor}, order_book_operations::OrderBooks}, market_handler::{MarketHandler}, orders::order::{ErrorCode, Order, OrderType}};

pub trait MarketOrders <E: Executor, O: OrderBooks, M: MarketHandler> {
    fn add_order(executor: &E, order: Order, matching: bool, recursive: bool, market_handler: &M) -> Result<(), ErrorCode>;
    fn add_market_order(executor: &E, order: Order, matching: bool, recursive: bool, market_handler: &M) -> Result<(), ErrorCode>;
}

impl<E: Executor, O: OrderBooks, M: MarketHandler> MarketOrders<E, O, M> for MarketExecutor {

    fn add_order(executor: &E, order: Order, matching: bool, recursive: bool, market_handler: &M) -> Result<(), ErrorCode> {
        order.validate();

        match order.order_type {
            OrderType::Buy => {
                // Handle Buy orders
                todo!()
            },
            OrderType::Market => {
                executor.add_market_order(market_handler, order, matching, recursive)
            },
            OrderType::Limit => {
                executor.add_limit_order(order, matching, recursive)
            },
            OrderType::Stop | OrderType::TrailingStop => {
                executor.add_stop_order(order, matching, recursive)
            },
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                executor.add_stop_limit_order(order, matching, recursive)
            },
            _ => Err(ErrorCode::OrderTypeInvalid),
        }
    }

    fn add_market_order(executor: &E, order_books: &O, order: Order, matching: bool, recursive: bool, market_handler: &M) -> Result<(), ErrorCode> {

        let mut order_book = order_books.get_order_book(&order.symbol_id);

        // market_handler.on_add_order(order);

        if matching && !recursive {
            executor.match_market(order_book, order);
        }

        // market_handler.on_delete_order(order);

        if matching && !recursive {
            executor.match_order_book(order_book, market_handler); // Assuming match_order also returns a Result
        }
        
        let mut order_book = order_book.reset_matching_price();

        Ok(())
    }
}