use crate::{orders::order::{ErrorCode, Order, OrderType}, order_book::order_book::BookOps, market_handler::Handler, levels::indexing::{LevelNode, RcNode}, references::Convertible};

use super::executor::Execution;


pub fn match_limit<E, B, C>(order_book: C, order: Order) 
where
    E: for<'a> Execution<'a>,
    B: for<'a> BookOps<'a>,
    C: Convertible<B>,{
    // Match the limit order
    E::match_order(order_book, order);
}

pub fn match_market<E, B, C>(mut order_book: C, mut order: Order) -> Result<(), ErrorCode> 
where
    E: for<'a> Execution<'a>,
    B: for<'a> BookOps<'a>,
    C: Convertible<B>,
{

    // Calculate acceptable market order price with optional slippage value
    match order.order_type {
        OrderType::Buy | OrderType::Market => {
            order.price = (*order_book.best_ask().expect("best ask not retrieved").borrow_mut()).price.saturating_add(order.slippage);
        },
        _ => {
            order.price = (*order_book.best_bid().expect("best bid not retrieved").borrow_mut()).price.saturating_sub(order.slippage);
        },
    }

    E::match_order(order_book, order);

    Ok(())
}

pub fn match_order<E, H, B, C>(mut order_book: C, mut order: Order) 
where
    E: for<'a> Execution<'a>,
    H: Handler,
    B: for<'a> BookOps<'a>,
    C: Convertible<B>,
    {
    
    let level_node: Option<RcNode>;
    let arbitrage = if order.is_buy() {
        level_node = order_book.best_ask();
        order.price >= (*level_node.expect("best ask not retrieved").borrow()).price
    } else {
        level_node = order_book.best_bid();
        order.price <= (*level_node.expect("best ask not retrieved").borrow()).price
    };  
    
    // Check the arbitrage bid/ask prices

    if !arbitrage {
        return;
    }

    // Special case for 'Fill-Or-Kill'/ll-Or-None' order
    if order.is_fok() || order.is_aon() {
        let chain = E::calculate_matching_chain_single_level(order_book, level_node, order.price, order.leaves_quantity);

        E::execute_matching_chain(order_book, level_node, order.price, chain);

        H::on_execute_order(&order, order.price, order.leaves_quantity);
        
        order_book.update_last_price(order, order.price);
        order_book.update_matching_price(order, order.price);
        
        order.executed_quantity += order.leaves_quantity;
        order.leaves_quantity = 0;

        return;
    }

    let mut executing_order = (*level_node.expect("best ask not retrieved").borrow()).orders.front();

    // Execute crossed E::orders
    while let Some(order_node) = executing_order {

        // get the execution quantity
        let quantity = order_node.leaves_quantity.min(order_node.leaves_quantity);

        // Special case for ll-Or-None' order_nodes
        if order_node.is_aon() && (order_node.leaves_quantity > order_node.leaves_quantity) {
            return;
        }

        // get the execution price
        let price = order_node.price;

        // Call the corresponding MarketHandler
        H::on_execute_order(&order_node.order, quantity, price);

        // Update the corresponding market price
        order_book.update_matching_price(order, order.price);

        // Increase the order executed quantity
        order.executed_quantity += quantity;

        // Reduce the executing order in the order book
        order_book.reduce_order(order_node, quantity, 0, 0);
        
        // Call the corresponding MarketHandler
        H::on_execute_order(&order, price, quantity);

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

pub fn match_order_book<E, B, H, C>(order_book: C, market_handler: H)
where
    E: for<'a> Execution<'a>,
    B: for<'a> BookOps<'a>,
    H: Handler,
    C: Convertible<B>
    {
    loop {
        // Check if the best bid price is higher than or equal to the best ask price
        while let (Some(bid_level_node), Some(ask_level_node)) = 
            (order_book.best_bid(), order_book.best_ask()) {
            // Break the loop if bid price is lower than ask price (no arbitrage opportunity)
            let (bid_level, ask_level) = (*bid_level_node.borrow(), ask_level_node.borrow());
            if bid_level.price < ask_level.price {
                break;
            }

            // Retrieve the front self.orders of both bid and ask levels
            let mut bid_order_node = bid_level.orders.front();
            let mut ask_order_node = ask_level.orders.front();

            // Process each pair of bid and ask self.orders
            while let (Some(bid_node_handle), Some(ask_node_handle)) = (bid_order_node, ask_order_node) {
                let next_bid_order_node = bid_node_handle.next_mut();
                let next_ask_order_node = ask_node_handle.next_mut();
                // Check for All-Or-None (AON) self.orders and handle them separately
                if bid_node_handle.is_aon() || ask_node_handle.is_aon() {
                    // Calculate the matching chain for AON self.orders
                    let chain = E::calculate_matching_chain_cross_levels(order_book, bid_level_node, ask_level_node);

                    // If no matching chain is found, exit the function
                    if chain == 0 {
                        return;
                    }

                    // Execute matching chains for AON self.orders
                    if bid_node_handle.is_aon() {
                        let price = bid_node_handle.price;
                        E::execute_matching_chain(order_book, bid_level, price, chain);
                        E::execute_matching_chain(order_book, ask_level, price, chain);
                    } else {
                        let price = ask_node_handle.price;
                        E::execute_matching_chain(order_book, ask_level, price, chain);
                        E::execute_matching_chain(order_book, bid_level, price, chain);
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
                H::on_execute_order(&executing_order.order, price, quantity);
                order_book.update_last_price(executing_order.order, price);
                order_book.update_matching_price(executing_order.order, price);
                
                // Update the executed order's quantity
                executing_order.executed_quantity += quantity;
                // Reduce the quantity of the executing order
                E::delete_order_recursive(executing_order.id, true, false);
                
                // Execute the reducing order
                H::on_execute_order(&reducing_order.order, price, quantity);
                
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
            
            E::activate_stop_orders_level(order_book.best_buy_stop(), order_book.get_market_ask_price());
            
            E::activate_stop_orders_level(order_book.best_sell_stop(), order_book.get_market_bid_price());
        }

        // Keep activating stop self.orders until no more can be activated
        if !E::activate_stop_orders(order_book) {
            break;
        }
    }
}
