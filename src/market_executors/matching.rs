use crate::{order_book::OrderBook, order::{ErrorCode, Order, OrderType}, market_managers::market_manager::MarketExecutor};



trait Matcher<M> {
    fn match_order_book(executor: &MarketExecutor, order_book: &OrderBook, market_handler: &MarketHandler);
    fn match_market(executor: &MarketExecutor, order_book: &OrderBook, order: Order) -> Result<(), ErrorCode>;
    fn match_limit(executor: &MarketExecutor, order_book: &OrderBook, order: Order);
    fn match_order(executor: &MarketExecutor, order_book: &OrderBook, order: Order);
}


impl Matcher<M> for MarketExecutor<MarketHandler> {
    fn match_limit(executor: &MarketExecutor, order_book: &OrderBook, order: Order) {
        // Match the limit order
        executor.match_order(order_book, order);
    }

    fn match_market(executor: &MarketExecutor, mut order_book: &OrderBook, mut order: Order) -> Result<(), ErrorCode> {

        // Calculate acceptable market order price with optional slippage value
        match order.order_type {
            OrderType::Buy | OrderType::Market => {
                order.price = order_book.best_ask.price.saturating_add(order.slippage);
            },
            _ => {
                order.price = order_book.best_bid.price.saturating_sub(order.slippage);
            },
        }

        executor.match_order(order_book, order);

        Ok(())
    }

    fn match_order(executor: &MarketExecutor, mut order_book: &OrderBook, mut order: Order, market_handler: &MarketHandler) {
        
        let level_node: LevelNode;
        if order.is_buy() {
            level_node = order_book.best_ask
        } else {
            level_node = order_book.best_bid
        };  
        
        // Check the arbitrage bid/ask prices
        let arbitrage = if order.is_buy() {
            order.price >= level_node.price
        } else {
            order.price <= level_node.price
        };

        if !arbitrage {
            return;
        }

        // Special case for 'Fill-Or-Kill'/ll-Or-None' order
        if order.is_fok() || order.is_aon() {
            let chain = executor.calculate_matching_chain_single_level(order_book, level_node, order.price, order.leaves_quantity);

            executor.execute_matching_chain(order_book, level_node, order.price, chain);

            market_handler.on_execute_order(order, order.price, order.leaves_quantity);
            
            order_book.update_last_price(order, order.price);
            order_book.update_matching_price(order, order.price);
            
            order.executed_quantity += order.leaves_quantity;
            order.leaves_quantity = 0;

            return;
        }

        let mut executing_order = level_node.orders.front();

        // Execute crossed executor.orders
        while let Some(order_node) = executing_order {

            // get the execution quantity
            let quantity = order_node.leaves_quantity.min(order_node.leaves_quantity);

            // Special case for ll-Or-None' order_nodes
            if order_node.is_aon() && (order_node.leaves_quantity > order_node.leaves_quantity) {
                return;
            }

            // get the execution price
            let price = order_node.price;

            // Call the corresponding handler
            market_handler.on_execute_order(&order_node.order, quantity, price);

            // Update the corresponding market price
            order_book.update_matching_price(order, order.price);

            // Increase the order executed quantity
            order.executed_quantity += quantity;

            // Reduce the executing order in the order book
            order_book.reduce_order(order_node, quantity, 0, 0);
            
            // Call the corresponding handler
            market_handler.on_execute_order(order, price, quantity);

            // Update the corresponding market price
            order_book.update_last_price(order, order.price);
            order_book.update_matching_price(order, order.price);

            // Increase the order executed quantity
            order.executed_quantity += quantity;

            // Reduce the order leaves quantity
            order.leaves_quantity -= quantity;
            if order.leaves_quantity == 0 {
                return;
            }
            
            let next_executing_order = order_node.next_mut();
            
            // Move to the next order to execute at the same price level
            if let Some(node) = next_executing_order {
                executing_order = Some(&node);
            } else {
                break;
            }
        }   
    }

    fn match_order_book<O, M>(executor: &MarketExecutor, order_book: &OrderBook, market_handler: &MarketHandler) {
        loop {
            // Check if the best bid price is higher than or equal to the best ask price
            while let (Some(bid_level_node), Some(ask_level_node)) = 
                (Some(order_book.best_bid()), Some(order_book.best_ask())) {
                // Break the loop if bid price is lower than ask price (no arbitrage opportunity)
                if bid_level_node.price < ask_level_node.price {
                    break;
                }

                // Retrieve the front self.orders of both bid and ask levels
                let mut bid_order_node = bid_level_node.orders.front();
                let mut ask_order_node = ask_level_node.orders.front();

                // Process each pair of bid and ask self.orders
                while let (Some(bid_node_handle), Some(ask_node_handle)) = (bid_order_node, ask_order_node) {
                    let next_bid_order_node = bid_node_handle.next_mut();
                    let next_ask_order_node = ask_node_handle.next_mut();
                    // Check for All-Or-None (AON) self.orders and handle them separately
                    if bid_node_handle.is_aon() || ask_node_handle.is_aon() {
                        // Calculate the matching chain for AON self.orders
                        let chain = executor.calculate_matching_chain_cross_levels(order_book, bid_level_node, ask_level_node);

                        // If no matching chain is found, exit the function
                        if chain == 0 {
                            return;
                        }

                        // Execute matching chains for AON self.orders
                        if bid_node_handle.is_aon() {
                            let price = bid_node_handle.price;
                            executor.execute_matching_chain(order_book, bid_level_node, price, chain);
                            executor.execute_matching_chain(order_book, ask_level_node, price, chain);
                        } else {
                            let price = ask_node_handle.price;
                            executor.execute_matching_chain(order_book, ask_level_node, price, chain);
                            executor.execute_matching_chain(order_book, bid_level_node, price, chain);
                        }
                        break;
                    }

                    // Determine which order to execute and which to reduce based on leaves quantity
                    let (mut executing_order, mut reducing_order) = if bid_node_handle.leaves_quantity > ask_node_handle.leaves_quantity {
                        (ask_node_handle, bid_node_handle)
                    } else {
                        (bid_node_handle, ask_node_handle)
                    };
                    
                    // Determine the quantity and price for execution
                    let quantity = executing_order.leaves_quantity;
                    let price = executing_order.price;
                    
                    // Execute the selected order
                    // market_handler.on_execute_order(&executing_order.order, price, quantity);
                    order_book.update_last_price(executing_order.order, price);
                    order_book.update_matching_price(executing_order.order, price);
                    
                    // Update the executed order's quantity
                    executing_order.executed_quantity += quantity;
                    // Reduce the quantity of the executing order
                    executor.delete_order_recursive(executing_order.id, true, false);
                    
                    // Execute the reducing order
                    // market_handler.on_execute_order(&reducing_order.order, price, quantity);
                    
                    order_book.update_last_price(reducing_order.order, price);
                    order_book.update_matching_price(reducing_order.order, price);
                    
                    // Update the reducing order's quantity
                    reducing_order.executed_quantity += quantity;

                    // Decrease the leaves quantity of the executing order
                    executing_order.leaves_quantity -= quantity;

                    // Move to the next pair of self.orders at the same level
                    bid_order_node = next_bid_order_node;
                    ask_order_node = next_ask_order_node;
                }

                let best_buy_stop = order_book.best_buy_stop();
                let market_ask_price = order_book.get_market_ask_price();
                
                executor.activate_stop_orders_level(order_book.best_buy_stop(), order_book.get_market_ask_price());
                
                let best_sell_stop = order_book.best_sell_stop();
                let market_bid_price = order_book.get_market_bid_price();
                
                executor.activate_stop_orders_level(order_book.best_sell_stop(), order_book.get_market_bid_price());
            }

            // Keep activating stop self.orders until no more can be activated
            if !executor.activate_stop_orders(order_book) {
                break;
            }
        }
    }
}
