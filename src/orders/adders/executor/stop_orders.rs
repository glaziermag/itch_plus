use crate::{market_handler::Handler, orders::{order::{ErrorCode, Order, OrderType, TimeInForce, OrderNode}, orders::Orders}, market_executors::{executor::{MarketExecutor, Execution}, order_book_operations::OrderBooks}};


pub fn add_stop_order<E: for<'a> Execution<'a>, H: Handler>(orders: &Orders, order_books: &OrderBooks, mut order: Order, matching: bool, recursive: bool, market_handler: H) -> Result<(), ErrorCode> {
    
    // remove panicking behavior from code
    let mut order_book = order_books.get_order_book(&order.symbol_id).expect("order book not found");

    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        order.stop_price = order_book.calculate_trailing_stop_price(order);
    }

    H::on_add_order(&order);

    if matching && !recursive {
        let stop_price = if order.is_buy() {
            order_book.get_market_ask_price()
        } else {
            order_book.get_market_bid_price()
        };

        let arbitrage = if order.is_buy() {
            order.stop_price <= stop_price
        } else {
            order.stop_price >= stop_price
        };

        if arbitrage {
            order.order_type = OrderType::Market;
            order.price = 0;
            order.stop_price = 0;
            order.time_in_force = if order.is_fok() {
                TimeInForce::FOK
            } else {
                TimeInForce::IOC
            };

            H::on_update_order(order);
            E::match_market(order_book, order);
            H::on_delete_order(order);
            if matching && !recursive {
                E::match_order_book(order_book, market_handler);
            }
            
            order_book.reset_matching_price();

            return Ok(());
        }
    }

    if order.leaves_quantity > 0 {
        let order_node = OrderNode::new(&order);
        if orders.insert(order_node.id, order_node).is_some() {
            // Order duplicate
            H::on_delete_order(order);
            //order_pool.release(order_node);
        }
    
        // Add the new stop order into the order book
        if order.is_trailing_stop() || order.is_trailing_stop_limit() {
            order_book.add_trailing_stop_order(&order_node)
        } else {
            order_book.add_stop_order(&order_node)
        }
    } else {
        H::on_delete_order(order);
    }

    if matching && !recursive {
        E::match_order_book(order_book, market_handler);
    }
    
    order_book.reset_matching_price();

    Ok(())
}

pub fn add_stop_limit_order<E: for<'a> Execution<'a>, H: Handler>(order_books: &OrderBooks, orders: &Orders, market_handler: H, mut order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> {

    // get the valid order book for the order
    let mut order_book = order_books.get_order_book(order.id);

    // Recalculate stop price for trailing stop self.orders
    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        let diff = order.price as i64 - order.stop_price as i64;
        let mut level_update = order_book.calculate_trailing_stop_price(order);
        order.price = (order.stop_price as i64 + diff) as u64;
    }

    // Call the corresponding MarketHandler
    H::on_add_order(&order);

    // Automatic order matching
    if matching && !recursive {
        // Find the price to match the stop-limit order
        let stop_price = if order.is_buy() {
            order_book.get_market_ask_price()
        } else {
            order_book.get_market_bid_price()
        };

        // Check the arbitrage bid/ask prices
        let arbitrage = if order.is_buy() {
            order.stop_price <= stop_price
        } else {
            order.stop_price >= stop_price
        };

        if arbitrage {
            // Convert the stop-limit order into the limit order
            order.order_type = OrderType::Limit;
            order.stop_price = 0;

            // Call the corresponding MarketHandler
            H::on_update_order(order);

            // Match the limit order
            E::match_limit(order_book, order);

            // Add a new limit order or delete remaining part in case of 'Immediate-Or-Cancel'/'Fill-Or-Kill' order
            if order.leaves_quantity > 0 && !order.is_ioc() && !order.is_fok() {
                // Create a new order
                let order_node = OrderNode::new(&order);
                if orders.insert(order_node.id, order_node).is_some() {
                    H::on_delete_order(order);
                    // order_pool.release(order_node);
                    // Handle duplicate order case here, if needed
                } else {
                    E::update_level(order_book.add_order(order_node));
                }
            } else {
                // Call the corresponding MarketHandler
                H::on_delete_order(&order);
            }

            // Automatic order matching
            if matching && !recursive {
                E::match_order_book(order_book, market_handler);
            }
            order_book.reset_matching_price();
        }
    }

    // Add a new stop order
    if order.leaves_quantity > 0 {
        // Insert the order
        let order_node = OrderNode::new(&order);
        if orders.insert(order_node.id, order_node).is_some() {
            // Order duplicate
            H::on_delete_order(order);
            // order_pool.release(// order_node.new(&order));
        }
        // Add the new stop order into the order book
        if order_node.is_trailing_stop() || order_node.is_trailing_stop_limit() {
            order_book.add_trailing_stop_order(order_node);
        } else {
            order_book.add_stop_order(order_node);
        }
    } else {
        // Call the corresponding MarketHandler
        H::on_delete_order(&order);
    }

    // Automatic order matching
    if matching && !recursive {
        E::match_order_book(order_book, market_handler);
    }

    order_book.reset_matching_price();

    Ok(())
}
