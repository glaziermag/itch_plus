

use crate::{orders::order::{Order, ErrorCode}, market_executors::executor::{MarketExecutor, Executor}, market_handler::MarketHandler};

use crate::market_executors::order_book_operations::OrderBooks;

trait AddLimit<E: Executor, O: OrderBooks, M: MarketHandler> {
    fn add_limit_order(&self, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
}

impl<E: Executor, O: OrderBooks, M: MarketHandler> AddLimit<E, O, M> for MarketExecutor {

    fn add_limit_order(executor: &E, order: Order, matching: bool, order_books: &O, recursive: bool, market_handler: &M) -> Result<(), ErrorCode> {
            
        let mut order_book = order_books.get_order_book(order.symbol_id);
        let order_node = executor.get_order_node(order.symbol_id);

        market_handler.on_add_order(&order);

        if matching && !recursive {
            executor.match_limit(order_book, order);
        }

        if (order.leaves_quantity > 0) && !order.is_ioc() && !order.is_fok() {
        // let order_node = order_node.new(&order);
            if executor.orders.insert(order_node.new(&order).id, order_node.new(&order)).is_some() {
                // Order duplicate
                market_handler.on_delete_duplicate_order(order_node);
               // order_pool.release(order_node.new(&order));
            } else {
                // Update level with the new order
                executor.update_level(order_book.add_order(order_node.new(&order)));
            }
        } else {
            market_handler.on_delete_unmatched_order(order);
        }

        if matching && !recursive {
            executor.match_order_book(order_book, market_handler);
        }

        order_book.reset_matching_price();

        Ok(())
    }
}