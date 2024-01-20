use crate::{order_book::order_book::OrderBook, levels::level::LevelNode, market_executors::executor::{MarketExecutor, Executor}, orders::{order::{OrderNode, OrderType, TimeInForce}, orders::Orders}};


pub trait StopActivator<E: Executor>: Executor {
    fn activate_stop_orders(executor: &E, order_book: &OrderBook) -> bool;
    fn activate_individual_stop_orders(executor: &E, order_book: &OrderBook, level_node: LevelNode, stop_price: u64) -> bool;
    fn activate_stop_order(executor: &E, order_book: &OrderBook, order_node: &OrderNode) -> bool;
    fn activate_stop_limit_order(executor: &E, order_book: &OrderBook, order_node: &OrderNode) -> bool;
}

impl<E: Executor> StopActivator<E> for MarketExecutor {

    fn activate_stop_orders(executor: &E, mut order_book: &OrderBook) -> bool {

        let mut result = false;
        let mut stop = false;

        while !stop {
            stop = true;

            // Try to activate buy stop self.orders
            if StopActivator::activate_individual_stop_orders(order_book, order_book.best_buy_stop(), order_book.get_market_ask_price())
                || E::activate_individual_stop_orders(executor, order_book, order_book.best_trailing_buy_stop(), order_book.get_market_ask_price()) {
                result = true;
                stop = false;
            }
            let best_ask = order_book.best_ask();
            
            // Recalculate trailing buy stop self.orders
            executor.recalculate_trailing_stop_price(executor, order_book, best_ask);

            // Try to activate sell stop self.orders
            if executor.activate_individual_stop_orders(executor, order_book, order_book.best_sell_stop(), order_book.get_market_bid_price())
                || executor.activate_individual_stop_orders(executor, order_book, order_book.best_trailing_sell_stop(), order_book.get_market_bid_price()) {
                result = true;
                stop = false;
            }

            let best_bid = order_book.best_bid();
            // Recalculate trailing sell stop self.orders
            executor.recalculate_trailing_stop_price(executor, order_book, best_bid);
        
        }
        result
    }

    fn activate_individual_stop_orders(executor: &E, order_book: &OrderBook, level_node: LevelNode, stop_price: u64) -> bool {

        let mut result = false;

        let arbitrage = if level_node.is_bid() {
            stop_price <= level_node.price
        } else {
            stop_price >= level_node.price
        };
        if !arbitrage {
            return false;
        }

        let mut activating_order_node = level_node.orders.front_mut();

        while let Some(order_node) = activating_order_node {

            let mut next_activating_order_node = order_node.next_mut();

            match order_node.order_type {
                OrderType::Stop | OrderType::TrailingStop => {
                    result |= executor.activate_stop_order(order_book, order_node);
                },
                OrderType::StopLimit | OrderType::TrailingStopLimit => {
                    result |= executor.activate_stop_limit_order(order_book, order_node);
                },
                _ => panic!("Unsupported order type!"),
            }
            activating_order_node = next_activating_order_node;
        }
        result
    }

    fn activate_stop_order(executor: &E, mut order_book: &OrderBook, mut order_node: &OrderNode) -> bool {
        // Delete the stop order from the order book
        if order_node.is_trailing_stop() || order_node.is_trailing_stop_limit() {
            order_book.delete_trailing_stop_order(order_node);
        } else {
            order_book.delete_stop_order(order_node);
        }

        // Convert the stop order into the market order
        order_node.order_type = OrderType::Market;
        order_node.price = 0;
        order_node.stop_price = 0;
        order_node.time_in_force = if order_node.is_fok() { TimeInForce::FOK } else { TimeInForce::IOC };

        // Call the corresponding handler
        // market_handler.on_update_order(&order_node.order);

        // Match the market order
        executor.match_market(order_book, order_node.order);

        // Call the corresponding handler
        // market_handler.on_delete_order_node(order_node);

        // Erase the order
        executor.orders.remove(&order_node.id);

        // Release the order, assuming we have an order pool with a release method
        // order_pool.release(order_node);

        true
    }

    fn activate_stop_limit_order<O: Orders>(executor: &E, mut order_book: &OrderBook, mut order_node: &OrderNode, orders: &O) -> bool {
        // Delete the stop order from the order book
        if order_node.is_trailing_stop() || order_node.is_trailing_stop_limit() {
            order_book.delete_trailing_stop_order(order_node);
        } else {
            order_book.delete_stop_order(order_node);
        }

        order_node.order_type = OrderType::Limit;
        order_node.stop_price = 0;

        // market_handler.on_update_order(&order_node.order);

        executor.match_limit(order_book, order_node.order);

        if order_node.leaves_quantity > 0 && !order_node.is_ioc() && !order_node.is_fok() {
            let level_update= order_book.add_order(order_node);
            executor.update_level(order_book, level_update);
        } else {
            // Call the corresponding handler
            //market_handler.on_delete_order(&order_node.order);
            orders.remove(&order_node.order.id);
            // order_pool.release(order_node);
        }
        true
    }
}
