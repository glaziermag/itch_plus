
extern crate generational_arena;


use std::cmp::min;
use std::collections::HashMap;


use crate::market_handler::MarketHandler;
use crate::order::OrderHandle;
use crate::{OrderType, UpdateType, LevelUpdate, LevelNodeHandle};
use crate::order::{ErrorCode, TimeInForce, OrderNodeHandle};
use crate::order_book::{OrderBook, OrderBookHandle};
//use crate::order_book_handle_pool::OrderBookPool;
use crate::order_pool::OrderPool;
use crate::level::LevelType;


fn add_order_book_handle(symbols: Vec<u64>, mut order_book_handles: HashMap<u64, OrderBookHandle>, symbol: u64) -> Result<(), ErrorCode> {
    // Check if the symbol exists
    if !symbols.contains(&symbol) {
        return Err(ErrorCode::SymbolNotFound);
    }

    // Check for existing OrderBook
    if order_book_handles.contains_key(&symbol) {
        return Err(ErrorCode::OrderBookDuplicate);
    }

    // Create a new OrderBook
    // Assuming OrderBook::new() does not require a weak reference to MarketManager
    let order_book_handle = OrderBookHandle::new(OrderBook::default());

    // Insert the new OrderBook into the HashMap
    order_book_handles.insert(symbol, order_book_handle);

    Ok(())
}

fn add_symbol(mut symbols: Vec<u64>, symbol: u64) -> Result<(), String> {
    // Assuming Symbol has an id field
    let usize_symbol_id: usize = symbol.try_into().unwrap();

    if usize_symbol_id >= symbols.len() {
        symbols.resize_with(usize_symbol_id + 1, Default::default);
    }

    if symbols.get(usize_symbol_id).is_some() {
        return Err("Symbol already exists".to_string());
    }

    symbols[usize_symbol_id] = symbol;

    Ok(())
}

fn insert_order_node(mut orders: HashMap<u64, OrderNodeHandle>, order_id: u64, order_node_handle: OrderNodeHandle) {
    orders.insert(order_id, order_node_handle);
} 

fn match_limit(
    order_pool: OrderPool,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler,
    orders: HashMap<u64, OrderNodeHandle>,
    order_book_handle: OrderBookHandle, 
    order_handle: OrderHandle
    ) {
    // Match the limit order
    match_order(order_pool, orders, market_handler, order_book_handles, order_book_handle, order_handle);
}

fn match_order(
    order_pool: OrderPool,
    orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    mut order_book_handle: OrderBookHandle, 
    mut order_handle: OrderHandle
    ) {
    
    let level_node_handle: LevelNodeHandle;
    if order_handle.clone().lock_unwrap().is_buy() {
        level_node_handle = order_book_handle.clone().lock_unwrap().best_ask.clone()
    } else {
        level_node_handle = order_book_handle.clone().lock_unwrap().best_bid.clone()
    };  
    {
    // Check the arbitrage bid/ask prices
    let arbitrage = if order_handle.clone().lock_unwrap().is_buy() {
        order_handle.clone().lock_unwrap().price >= level_node_handle.clone().lock_unwrap().price
    } else {
        order_handle.clone().lock_unwrap().price <= level_node_handle.clone().lock_unwrap().price
    };

    if !arbitrage {
        return;
    }

    // Special case for 'Fill-Or-Kill'/ll-Or-None' order
    if order_handle.clone().lock_unwrap().is_fok() || order_handle.clone().lock_unwrap().is_aon() {
        let chain = calculate_matching_chain_single_level(order_book_handle.clone(), level_node_handle.clone(), order_handle.clone().lock_unwrap().price, order_handle.clone().lock_unwrap().leaves_quantity);

        execute_matching_chain(order_pool, orders, order_book_handles, market_handler, order_book_handle.clone(), level_node_handle.clone(), order_handle.clone().lock_unwrap().price, chain);

        // market_handler.on_execute_order(order_handle, order_handle.clone().lock_unwrap().price, order_handle.clone().lock_unwrap().leaves_quantity);
        
        order_book_handle.clone().lock_unwrap().update_last_price(order_handle.clone(), order_handle.clone().lock_unwrap().price);
        order_book_handle.clone().lock_unwrap().update_matching_price(order_handle.clone(), order_handle.clone().lock_unwrap().price);
        

        order_handle.clone().lock_unwrap().executed_quantity += order_handle.clone().lock_unwrap().leaves_quantity;
        order_handle.clone().lock_unwrap().leaves_quantity = 0;

        return;
    }

    let level_node_handle_clone = level_node_handle.clone();
    let level_node_locked = level_node_handle_clone.lock_unwrap();
    let mut executing_order_handle = level_node_locked.order_list.front().map(|node| node.clone());

    // Execute crossed orders
    while let Some(order_node_handle) = executing_order_handle {

        // get the execution quantity
        let quantity = order_node_handle.clone().lock_unwrap().leaves_quantity.min(order_node_handle.clone().lock_unwrap().leaves_quantity);

        // Special case for ll-Or-None' order_nodes
        if order_node_handle.clone().lock_unwrap().is_aon() && (order_node_handle.clone().lock_unwrap().leaves_quantity > order_node_handle.clone().lock_unwrap().leaves_quantity) {
            return;
        }

        // get the execution price
        let price = order_node_handle.clone().lock_unwrap().price;

        // Call the corresponding handler
        // market_handler.on_execute_order_handle(&order_node_handle.clone().lock_unwrap().order_handle, quantity, price);

        // Update the corresponding market price
        {
            let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handle_locked.update_last_price(order_handle.clone(), order_handle.clone().lock_unwrap().price);
            order_book_handle_locked.update_matching_price(order_handle.clone(), order_handle.clone().lock_unwrap().price);
        }

        // Increase the order executed quantity
        order_handle.clone().lock_unwrap().executed_quantity += quantity;

        // Reduce the executing order in the order book
        {
            let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handle_locked.reduce_order(order_node_handle.clone(), quantity, 0, 0);
        }

        // Call the corresponding handler
        // market_handler.on_execute_order(order_handle, price, quantity);

        // Update the corresponding market price
        {
            let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handle_locked.update_last_price(order_handle.clone(), order_handle.clone().lock_unwrap().price);
            order_book_handle_locked.update_matching_price(order_handle.clone(), order_handle.clone().lock_unwrap().price);
        }

        // Increase the order executed quantity
        order_handle.clone().lock_unwrap().executed_quantity += quantity;

        // Reduce the order leaves quantity
        order_handle.clone().lock_unwrap().leaves_quantity -= quantity;
        if order_handle.clone().lock_unwrap().leaves_quantity == 0 {
            return;
        }

        // Find the next order to execute
        let next_executing_order_handle: Option<OrderNodeHandle>;
        
         
        let order_node_handle_clone = order_node_handle.clone();
        let order_node_handle_locked = order_node_handle_clone.lock_unwrap();
        next_executing_order_handle = order_node_handle_locked.next_mut();
        
        // Move to the next order to execute at the same price level
        if let Some(order_handle) = next_executing_order_handle {
            executing_order_handle = Some(order_handle);
        } else {
            break;
        }
        }
    }
}

fn calculate_matching_chain_single_level(mut order_book_handle: OrderBookHandle, level_node_handle: LevelNodeHandle, price: u64, volume: u64) -> u64 {

    let mut available = 0;
    let binding = level_node_handle.clone().lock_unwrap().clone();
    let order_node = binding.front();
    let mut order_node_handle = order_node.map(|node| node.clone());
    let mut level_node_handle = Some(level_node_handle.clone());

    
    while let Some(level_handle) = level_node_handle {

        // Check the arbitrage bid/ask prices
        let arbitrage = if level_handle.clone().lock_unwrap().is_bid() {
            price <= level_handle.clone().lock_unwrap().price
        } else {
            price >= level_handle.clone().lock_unwrap().price
        };

        if !arbitrage {
            return 0;
        }
        
        // Travel through orders at current price levels

        while let Some(ref order_node) = order_node_handle {

            let need = volume - available;
            //let order_node_locked = order_node.clone().lock_unwrap();

            let quantity = if order_node.clone().lock_unwrap().is_aon() {
                order_node.clone().lock_unwrap().leaves_quantity
            } else {
                std::cmp::min(order_node.clone().lock_unwrap().leaves_quantity, need)
            };
            available += quantity;

            // Matching is possible, return the chain size
            if volume == available {
                return available;
            }

            // Matching is not possible
            if volume < available {
                return 0;
            }

            // Take the next order
            // Clone and lock `order_node`, storing the result in a variable
            let cloned_node = order_node.clone();
            let locked_node = cloned_node.lock_unwrap();

            // // Now you can safely call `next_mut()` on the locked node
            let next_node = locked_node.next_mut();

            if let Some(next_node_handle) = next_node {
                order_node_handle = Some(next_node_handle.clone());
            } else {
                break;
            }
        }

        // Switch to the next price level
        let order_book_locked = order_book_handle.lock_unwrap();
        if let Some(next_level_handle) = order_book_locked.get_next_level(level_handle) {
            level_node_handle = Some(next_level_handle.clone());
        } else {
            break;
        }
    }
    // Matching is not available
    0
}

fn execute_matching_chain(
    order_pool: OrderPool,
    orders: HashMap<u64, OrderNodeHandle>,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler,
    mut order_book_handle: OrderBookHandle, 
    level_node_handle: LevelNodeHandle, 
    price: u64, 
    mut volume: u64
    ) {

    let mut level_node_handle = Some(level_node_handle.clone());

    while volume > 0 {
        if let Some(ref mut current_level_node_handle) = level_node_handle.clone() {
            let current_level_node_handle_clone = current_level_node_handle.clone();
            let current_level_node_locked = current_level_node_handle_clone.lock_unwrap();
            let mut executing_order_handle = current_level_node_locked.order_list.front().map(|node| node.clone());

            while volume > 0 {
                if let Some(order_node_handle) = executing_order_handle {
                    let quantity = if order_node_handle.clone().lock_unwrap().is_aon() {
                        order_node_handle.clone().lock_unwrap().leaves_quantity
                    } else {
                        std::cmp::min(order_node_handle.clone().lock_unwrap().leaves_quantity, volume)
                    };

                    // market_handler.on_execute_order(&order_node_handle.clone().lock_unwrap().order_handle, price, quantity);
                    // Switch to the next price level
                    {
                        let mut order_book_handle_locked = order_book_handle.lock_unwrap();
                        order_book_handle_locked.update_last_price(order_node_handle.clone().lock_unwrap().order_handle.clone(), price);
                        order_book_handle_locked.update_matching_price(order_node_handle.clone().lock_unwrap().order_handle.clone(), price);
                    }

                    order_node_handle.clone().lock_unwrap().executed_quantity += quantity;
                    // Reduce the executing order in the order book
                    reduce_order(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_node_handle.clone().lock_unwrap().id, quantity, true, false);

                    volume -= quantity;
                    executing_order_handle = order_node_handle.clone().lock_unwrap().next_mut();
                } else {
                    break;
                }
            }// Assuming `get_next_level` returns an LevelNode
            {
                let order_book_handle_locked = order_book_handle.lock_unwrap();
                if let Some(next_level_node_handle) = order_book_handle_locked.get_next_level(current_level_node_handle.clone()) {
                    level_node_handle = Some(next_level_node_handle);
                    
                } else {
                }
            }
        } else {
            break;
        }
    }
}

fn activate_stop_limit_order(
    order_book_handles: HashMap<u64, OrderBookHandle>,
    mut order_pool: OrderPool,
    mut order_book_handle: OrderBookHandle, 
    mut orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler,  
    mut order_node_handle: OrderNodeHandle, 
    ) -> bool {
    // Delete the stop order from the order book
    if order_node_handle.clone().lock_unwrap().is_trailing_stop() || order_node_handle.clone().lock_unwrap().is_trailing_stop_limit() {
        let mut order_book_handle_locked = order_book_handle.lock_unwrap();
        order_book_handle_locked.delete_trailing_stop_order(order_node_handle.clone());
    } else {
        let mut order_book_handle_locked = order_book_handle.lock_unwrap();
        order_book_handle_locked.delete_stop_order(order_node_handle.clone());
    }

    order_node_handle.clone().lock_unwrap().order_type = OrderType::Limit;
    order_node_handle.clone().lock_unwrap().stop_price = 0;

    // market_handler.on_update_order(&order_node_handle.clone().lock_unwrap().order_handle);

    match_limit(order_pool.clone(), order_book_handles, market_handler.clone(), orders.clone(), order_book_handle.clone(), order_node_handle.clone().lock_unwrap().order_handle.clone());

    if order_node_handle.clone().lock_unwrap().leaves_quantity > 0 && !order_node_handle.clone().lock_unwrap().is_ioc() && !order_node_handle.clone().lock_unwrap().is_fok() {
        let level_update= order_book_handle.clone().lock_unwrap().add_order(order_node_handle);
        update_level(market_handler.clone(), order_book_handle.clone(), level_update);
    } else {
        // Call the corresponding handler
        // market_handler.on_delete_order(&order_node_handle.clone().lock_unwrap().order_handle);
        orders.clone().remove(&order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().id);
        order_pool.clone().release(order_node_handle);
    }
    true
}
          

fn add_order(
    order_pool: OrderPool,
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    mut orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler,
    order_handle: OrderHandle, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {
    order_handle.clone().lock_unwrap().validate();
    match order_handle.clone().lock_unwrap().order_type {
        OrderType::Buy => todo!(),
        OrderType::Market => add_market_order(order_book_handles, orders, order_pool, market_handler, order_handle, true, false),
        OrderType::Limit => add_limit_order(order_book_handles, order_pool, orders,  market_handler, order_handle, true, false),
        OrderType::Stop | OrderType::TrailingStop => add_stop_order(order_pool, orders, order_book_handles, market_handler, order_handle, true, false),
        OrderType::StopLimit | OrderType::TrailingStopLimit => add_stop_limit_order(order_pool, orders, order_book_handles, market_handler, order_handle, true, false),
        _ => Err(ErrorCode::OrderTypeInvalid),
    }
}

fn add_stop_limit_order(
    mut order_pool: OrderPool, 
    mut orders: HashMap<u64, OrderNodeHandle>,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler, 
    mut order_handle: OrderHandle, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {

    // get the valid order book for the order
    let mut order_book_handle = order_book_handles.get(&order_handle.clone().lock_unwrap().symbol_id)
        .ok_or(ErrorCode::OrderBookNotFound)?;

    // let order = order_handle.clone().lock_unwrap().clone();

    // Recalculate stop price for trailing stop orders
    if order_handle.clone().lock_unwrap().is_trailing_stop() || order_handle.clone().lock_unwrap().is_trailing_stop_limit() {
        let diff = order_handle.clone().lock_unwrap().price as i64 - order_handle.clone().lock_unwrap().stop_price as i64;
        {
            let mut level_update = order_book_handle.clone().lock_unwrap().calculate_trailing_stop_price(order_handle.clone());
        }
        order_handle.clone().lock_unwrap().price = (order_handle.clone().lock_unwrap().stop_price as i64 + diff) as u64;
    }

    // Call the corresponding handler
    // market_handler.on_add_order(&order_handle);

    // Automatic order matching
    if matching && !recursive {
        // Find the price to match the stop-limit order
        let stop_price = if order_handle.clone().lock_unwrap().is_buy() {
            order_book_handle.clone().lock_unwrap().get_market_ask_price()
        } else {
            order_book_handle.clone().lock_unwrap().get_market_bid_price()
        };

        // Check the arbitrage bid/ask prices
        let arbitrage = if order_handle.clone().lock_unwrap().is_buy() {
            order_handle.clone().lock_unwrap().stop_price <= stop_price
        } else {
            order_handle.clone().lock_unwrap().stop_price >= stop_price
        };

        if arbitrage {
            // Convert the stop-limit order into the limit order
            order_handle.clone().lock_unwrap().order_type = OrderType::Limit;
            order_handle.clone().lock_unwrap().stop_price = 0;

            // Call the corresponding handler
            // market_handler.on_update_order(order_handle);

            // Match the limit order
            match_limit(order_pool.clone(), order_book_handles.clone(), market_handler.clone(), orders.clone(), order_book_handle.clone(), order_handle.clone());

            // Add a new limit order or delete remaining part in case of 'Immediate-Or-Cancel'/'Fill-Or-Kill' order
            if order_handle.clone().lock_unwrap().leaves_quantity > 0 && !order_handle.clone().lock_unwrap().is_ioc() && !order_handle.clone().lock_unwrap().is_fok() {
                // Create a new order
                let order_node_handle= order_pool.clone().create(&order_handle).map(|node| node.clone()).unwrap();
                if orders.clone().insert(order_node_handle.clone().lock_unwrap().id, order_node_handle.clone()).is_some() {
                    // market_handler.on_delete_order(order_handle);
                    order_pool.release(order_node_handle.clone());
                    // Handle duplicate order case here, if needed
                } else {
                   // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                    update_level(market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().add_order(order_node_handle.clone()));
                }
            } else {
                // Call the corresponding handler
                // market_handler.on_delete_order(&order_handle);
            }

            // Automatic order matching
            if matching && !recursive {
                match_order_book_handle(order_book_handles.clone(), order_pool.clone(), order_book_handle.clone(), orders.clone(), market_handler.clone());
            }
            order_book_handle.clone().lock_unwrap().reset_matching_price();
        }
    }

    // Add a new stop order
    if order_handle.clone().lock_unwrap().leaves_quantity > 0 {
        // Insert the order
       // let order_node_handle = order_pool.clone().create(&order_handle).map(|node| node).unwrap();
        if orders.clone().insert(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone().lock_unwrap().id, 
            order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone()).is_some() {
            // Order duplicate
            // market_handler.on_delete_order(order_handle);
            order_pool.release(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone());
        }
    
        // Add the new stop order into the order book
        if order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone().lock_unwrap().is_trailing_stop() || order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone().lock_unwrap().is_trailing_stop_limit() {
            //let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            order_book_handle.clone().lock_unwrap().add_trailing_stop_order(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone());
        } else {
           // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            order_book_handle.clone().lock_unwrap().add_stop_order(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone());
        }
    } else {
        // Call the corresponding handler
        // market_handler.on_delete_order(&order_handle);
    }

    // Automatic order matching
    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool, order_book_handle.clone(), orders, market_handler.clone());
    }

    order_book_handle.clone().lock_unwrap().reset_matching_price();

    Ok(())
}

fn add_market_order(
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    orders: HashMap<u64, OrderNodeHandle>, 
    order_pool: OrderPool,
    market_handler: MarketHandler,
    order_handle: OrderHandle,
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {

    let mut order_book_handle = order_book_handles.get(&order_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    //let mut order = order_handle.clone().lock_unwrap().clone();

    // market_handler.on_add_order(order_handle);

    if matching && !recursive {
        match_market(order_pool.clone(), order_book_handles.clone(), order_book_handle.clone(), market_handler.clone(), orders.clone(), order_handle.clone());
    }

    // market_handler.on_delete_order(order_handle);

    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool.clone(), order_book_handle.clone(), orders.clone(), market_handler.clone()); // Assuming match_order also returns a Result
    }
    
    let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap().reset_matching_price();

    Ok(())
}

fn add_limit_order(
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    mut order_pool: OrderPool, 
    mut orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler,
    order_handle: OrderHandle, 
    matching: bool, 
    recursive: bool
    ) -> Result<(), ErrorCode> {
    
    let mut order_book_handle = order_book_handles.get(&order_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    // let order = order_handle.clone().lock_unwrap().clone();

    //// market_handler.on_add_order(&order_handle);

    if matching && !recursive {
        match_limit(order_pool.clone(), order_book_handles.clone(), market_handler.clone(), orders.clone(), order_book_handle.clone(), order_handle.clone());
    }

    if (order_handle.clone().lock_unwrap().leaves_quantity > 0) && !order_handle.clone().lock_unwrap().is_ioc() && !order_handle.clone().lock_unwrap().is_fok() {
       // let order_node_handle = order_pool.clone().create(&order_handle).map(|node| node).unwrap();
        if orders.clone().insert(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone().lock_unwrap().id, order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone()).is_some() {
            // Order duplicate
            // market_handler.on_delete_duplicate_order(order_node);
            order_pool.release(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone());
        } else {
            // Update level with the new order
            //let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            update_level(market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().add_order(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone()));
        }
    } else {
        // market_handler.on_delete_unmatched_order(order_handle);
    }

    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool, order_book_handle.clone(), orders, market_handler);
    }

    order_book_handle.clone().lock_unwrap().reset_matching_price();
   

    Ok(())
}

fn add_stop_order(
    mut order_pool: OrderPool,
    mut orders: HashMap<u64, OrderNodeHandle>, 
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    market_handler: MarketHandler,
    mut order_handle: OrderHandle, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {
    let mut order_book_handle = order_book_handles.get(&order_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    //let order_handle: OrderHandle = order_handle.clone().lock_unwrap().clone();

    if order_handle.clone().lock_unwrap().is_trailing_stop() || order_handle.clone().lock_unwrap().is_trailing_stop_limit() {
        order_handle.clone().lock_unwrap().stop_price = order_book_handle.clone().lock_unwrap().calculate_trailing_stop_price(order_handle.clone());
    }

    // market_handler.on_add_order(order_handle);

    if matching && !recursive {
        let stop_price = if order_handle.clone().lock_unwrap().is_buy() {
            order_book_handle.clone().lock_unwrap().get_market_ask_price()
        } else {
            order_book_handle.clone().lock_unwrap().get_market_bid_price()
        };

        let arbitrage = if order_handle.clone().lock_unwrap().is_buy() {
            order_handle.clone().lock_unwrap().stop_price <= stop_price
        } else {
            order_handle.clone().lock_unwrap().stop_price >= stop_price
        };

        if arbitrage {
            order_handle.clone().lock_unwrap().order_type = OrderType::Market;
            order_handle.clone().lock_unwrap().price = 0;
            order_handle.clone().lock_unwrap().stop_price = 0;
            order_handle.clone().lock_unwrap().time_in_force = if order_handle.clone().lock_unwrap().is_fok() {
                TimeInForce::FOK
            } else {
                TimeInForce::IOC
            };

            // market_handler.on_update_order(order_handle);
            match_market(order_pool.clone(), order_book_handles.clone(), order_book_handle.clone(), market_handler.clone(), orders.clone(), order_handle.clone());
            // market_handler.on_delete_order(order_handle);
            if matching && !recursive {
                match_order_book_handle(order_book_handles.clone(), order_pool.clone(), order_book_handle.clone(), orders.clone(), market_handler.clone());
            }
            
            order_book_handle.clone().lock_unwrap().reset_matching_price();

            return Ok(());
        }
    }

    if order_handle.clone().lock_unwrap().leaves_quantity > 0 {
        let order_node_handle = order_pool.clone().create(&order_handle).map(|node| node).unwrap();
        if orders.insert(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone().lock_unwrap().id, order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone()).is_some() {
            // Order duplicate
            // market_handler.on_delete_order(order_handle);
            order_pool.clone().release(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone());
        }
    
        // Add the new stop order into the order book
        if order_handle.clone().lock_unwrap().is_trailing_stop() || order_handle.clone().lock_unwrap().is_trailing_stop_limit() {
            order_book_handle.clone().lock_unwrap().add_trailing_stop_order(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone())
        } else {
            order_book_handle.clone().lock_unwrap().add_stop_order(order_pool.clone().create(&order_handle).map(|node| node).unwrap().clone())
        }
    } else {
        // market_handler.on_delete_order(order_handle);
    }

    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool.clone(), order_book_handle.clone(), orders.clone(), market_handler.clone());
    }
    
    order_book_handle.clone().lock_unwrap().reset_matching_price();

    Ok(())
}

fn reduce_order(
    mut order_pool: OrderPool, 
    mut orders: HashMap<u64, OrderNodeHandle>,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler, 
    id: u64, 
    quantity: u64, 
    matching: bool,
    recursive: bool
) -> Result<(), ErrorCode> {
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }
    let order_node_handle = orders.get(&id).ok_or(ErrorCode::OrderNotFound)?;
    let mut order_book_handle = order_book_handles.get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    let quantity = min(quantity, order_node_handle.clone().lock_unwrap().leaves_quantity);
    order_node_handle.clone().lock_unwrap().leaves_quantity -= quantity;

    let hidden = order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().hidden_quantity();
    let visible = order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().visible_quantity();

    // Update the order or delete the empty order
    if order_node_handle.clone().lock_unwrap().leaves_quantity > 0 {
        // market_handler.on_update_order(&order_node_handle.clone().lock_unwrap().order_handle);
        let order_node_handle = order_pool.create(&order_node_handle.clone().lock_unwrap().order_handle).map(|node| node).unwrap();

        match order_node_handle.clone().lock_unwrap().order_type {

            OrderType::Limit => {
                order_book_handle.clone().lock_unwrap().reduce_trailing_stop_order(order_node_handle.clone(), quantity, hidden, visible);
            },
            OrderType::Stop | OrderType::StopLimit => {
                order_book_handle.clone().lock_unwrap().reduce_trailing_stop_order(order_node_handle.clone(), quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                order_book_handle.clone().lock_unwrap().reduce_trailing_stop_order(order_node_handle.clone(), quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        }
    } else {
        // market_handler.on_delete_order(&order_node_handle.clone().lock_unwrap().order_handle);

        match order_node_handle.clone().lock_unwrap().order_type {
            OrderType::Limit => {
                update_level(market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().reduce_order(order_node_handle.clone(), quantity, hidden, visible));
            },
            OrderType::Stop | OrderType::StopLimit => {
                order_book_handle.clone().lock_unwrap().reduce_stop_order(order_node_handle.clone(), quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                order_book_handle.clone().lock_unwrap().reduce_trailing_stop_order(order_node_handle.clone(), quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        }

        // Erase the order
        orders.clone().remove(&id);

        // Release the order_handle, assuming we have an order pool with a release method
        order_pool.release(order_node_handle.clone());
    }

    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool, order_book_handle.clone(), orders, market_handler.clone());
    }
    
    order_book_handle.clone().lock_unwrap().reset_matching_price();
    

    Ok(())
}

fn mitigate_order(
    order_pool: OrderPool, 
    orders: HashMap<u64, OrderNodeHandle>,
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    market_handler: MarketHandler,
    id: u64, 
    new_price: u64, 
    new_quantity: u64
) -> Result<(), ErrorCode> {
    modify_order(order_pool, orders, market_handler, order_book_handles, id, new_price, new_quantity, true, true, false)
}


fn modify_order(
    mut order_pool: OrderPool,
    mut orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler,
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    id: u64, new_price: u64, 
    new_quantity: u64, 
    mitigate: bool, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if new_quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }

    let mut order_node_handle = orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?;

   // let mut order_book_handle = order_book_handles.clone().get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    match orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().order_type {
        OrderType::Limit => {
           // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            update_level(market_handler.clone(), order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone(), 
            order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_order(orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone()))
        },
        OrderType::Stop | OrderType::StopLimit => {
            Ok(order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_stop_order(orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone()))
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_trailing_stop_order(orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone())
        },
        _ => return Err(ErrorCode::OrderTypeInvalid),
    };

    // Modify the order
    orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().price = new_price;
    orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().quantity = new_quantity;
    orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().leaves_quantity = new_quantity;

    // In-Flight Mitigation (IFM)
    if mitigate {
        orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().leaves_quantity = if new_quantity > orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().executed_quantity {
            new_quantity - orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().executed_quantity
        } else {
            0
        };
    }

    // Update the order
    if orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().leaves_quantity > 0 {
        // market_handler.on_update_order(&order_node_handle.clone().lock_unwrap().order_handle);

        // Automatic order matching
        if matching && !recursive {
            match_limit(order_pool.clone(), order_book_handles.clone(), market_handler.clone(), orders.clone(), order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone(), orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().order_handle.clone());
        }

        // Add non-empty order into the order book
        if orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().leaves_quantity > 0 {
            match orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().order_type {
                OrderType::Limit => {
                  //  let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                    update_level(market_handler.clone(), order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone(), order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().add_order(orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone()));
                },
                OrderType::Stop | OrderType::StopLimit => {
                  // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                  order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().add_stop_order(orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
                },
                OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                  //  let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                  order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().add_trailing_stop_order(orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
                },
                _ => return Err(ErrorCode::OrderTypeInvalid),
            };
        }
    }

    // Delete the empty order
    if orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().leaves_quantity == 0 {
        // market_handler.on_delete_order(&order_node_handle.clone().lock_unwrap().order_handle);
        orders.clone().clone().remove(&id);
        order_pool.clone().release(orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
    }

    // Automatic order matching
    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool, order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone(), orders.clone(), market_handler.clone());
    }

    // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
    // Reset matching price
    order_book_handles.clone().get(&orders.clone().get(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().reset_matching_price();

    Ok(())
}

fn replace_order_id(
    mut order_pool: OrderPool, 
    mut orders: HashMap<u64, OrderNodeHandle>,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler,
    symbols: Vec<u64>,
    id: u64, 
    new_id: u64, 
    new_price: u64, 
    new_quantity: u64
) -> Result<(), ErrorCode> {
    replace_order_internal(order_pool, orders, market_handler, order_book_handles, symbols, id, new_id, new_price, new_quantity, false, false)
}

fn replace_order_internal(
    mut order_pool: OrderPool,
    mut orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler, 
    order_book_handles: HashMap<u64, OrderBookHandle>,
    symbols: Vec<u64>,
    id: u64, 
    new_id: u64, 
    new_price: u64, 
    new_quantity: u64, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {
    // Validate parameters 
    if id == 0 || new_id == 0 || new_quantity == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // get the order to replace 
   //let order_node_handle = orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?;
    if !orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().is_limit() {
        return Err(ErrorCode::OrderTypeInvalid);
    }

    // get the valid order book for the order 
    let mut order_book_handle = order_book_handles.get(&orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    // Reduce the order leaves quantity
    // Reduce the order in the order book
    match orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().order_type {
        OrderType::Limit => {
            //let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            update_level(market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().delete_order(orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone()));
        },
        OrderType::Stop | OrderType::StopLimit => {
            //let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handles.clone().get(&orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_stop_order(orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            //let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handles.clone().get(&orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_trailing_stop_order(orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
        },
        _ => return Err(ErrorCode::OrderTypeInvalid),
    }

    // Call the corresponding handler 
    // market_handler.on_delete_order_node(order_node);

    // Erase the order 
    orders.clone().remove(&id);

    // Replace the order 
    orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().id = new_id;
    orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().price = new_price;
    orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().quantity = new_quantity;
    orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().executed_quantity = 0;
    orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().leaves_quantity = new_quantity;

    // Call the corresponding handler 
    // market_handler.on_add_order_node(order_node);

    // Automatic order matching 
    if matching && !recursive {
        match_limit(order_pool.clone(), order_book_handles.clone(), market_handler.clone(), orders.clone(), order_book_handle.clone(), orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().order_handle.clone());
    }

    if orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone().lock_unwrap().leaves_quantity > 0 {
        // Insert the order 
        if orders.clone().insert(new_id, orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone()).is_some() {
            order_pool.clone().release(orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
            return Err(ErrorCode::OrderDuplicate);
        }
        // Add the modified order into the order book 
        add_order_book_handle(symbols, order_book_handles.clone(), new_id)?;
    } else {
        // market_handler.on_delete_order_node(order_node);
        // Release the order if empty 
        order_pool.release(orders.clone().get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
    }

    // Automatic order matching 
    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool, order_book_handle.clone(), orders, market_handler);
    }

    // Reset matching price 
    {
        //let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
        order_book_handle.clone().lock_unwrap().reset_matching_price();
    }
    Ok(())
}

fn replace_order(
    order_pool: OrderPool, 
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    orders: HashMap<u64, OrderNodeHandle>, 
    market_handler: MarketHandler,
    id: u64, 
    order_handle: OrderHandle, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {
    // Delete the previous order by Id
    let result = delete_order(orders.clone(), order_book_handles.clone(), market_handler.clone(), order_pool.clone(), id.try_into().unwrap(), true, false);
    if let Err(error) = result {
        return Err(error);
    }

    add_order(order_pool.clone(), order_book_handles.clone(), orders.clone(), market_handler.clone(), order_handle, matching, recursive)
}

fn execute_order(
    mut order_pool: OrderPool,
    order_book_handles: HashMap<u64, OrderBookHandle>, 
    mut orders: HashMap<u64, OrderNodeHandle>, 
    mut market_handler: MarketHandler,
    id: u64, 
    price: u64, 
    quantity: u64, 
    matching: bool
) -> Result<(), ErrorCode> {
    assert!(id > 0, "Order Id must be greater than zero!");
    assert!(quantity > 0, "Order quantity must be greater than zero!");

    let order_node_handle = orders.get_mut(&id).ok_or(ErrorCode::OrderNotFound)?;

    let mut order_book_handle = order_book_handles.get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    let quantity = std::cmp::min(quantity, order_node_handle.clone().lock_unwrap().leaves_quantity);
    // market_handler.on_execute_order_node(order_node, order_node_handle.clone().lock_unwrap().price, quantity);
    {
       // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
        order_book_handle.clone().lock_unwrap().update_last_price(order_node_handle.clone().lock_unwrap().order_handle.clone(), order_node_handle.clone().lock_unwrap().price);
        order_book_handle.clone().lock_unwrap().update_matching_price(order_node_handle.clone().lock_unwrap().order_handle.clone(), order_node_handle.clone().lock_unwrap().price);
    }

    let hidden = order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().hidden_quantity();
    let visible = order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().visible_quantity();
    order_node_handle.clone().lock_unwrap().executed_quantity += quantity;
    order_node_handle.clone().lock_unwrap().leaves_quantity -= quantity;

    let hidden_delta = hidden - order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().hidden_quantity();
    let visible_delta = visible - order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().visible_quantity();

    match order_node_handle.clone().lock_unwrap().order_type {
        OrderType::Limit => {
           // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            update_level(market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().reduce_order(order_node_handle.clone(), quantity, hidden, visible));
        },
        OrderType::Stop | OrderType::StopLimit => {
            //let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handle.clone().lock_unwrap().reduce_stop_order(order_node_handle.clone(), quantity, hidden_delta, visible_delta);
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            // let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handle.clone().lock_unwrap().reduce_trailing_stop_order(order_node_handle.clone(), quantity, hidden_delta, visible_delta);
        },
        OrderType::Buy => todo!(),
        OrderType::Market => todo!(),
    }

    if order_node_handle.clone().lock_unwrap().leaves_quantity > 0 {
        // market_handler.on_update_order(&order_node_handle.clone().lock_unwrap().order_handle);
    } else {
        // market_handler.on_delete_order_node(order_node);
        orders.clone().remove(&id);
        order_pool.release(orders.get_mut(&id).ok_or(ErrorCode::OrderNotFound)?.clone());
    }

    if matching {
        match_order_book_handle(order_book_handles.clone(), order_pool, order_book_handle.clone(), orders, market_handler.clone());
    }
    order_book_handle.clone().lock_unwrap().reset_matching_price();

    Ok(())
}

// "match", automatic order match
fn match_order_book_handle(
    order_book_handles: HashMap<u64, OrderBookHandle>,
    mut order_pool: OrderPool,
    mut order_book_handle: OrderBookHandle,
    mut orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler
    ) {


    loop {
        // Check if the best bid price is higher than or equal to the best ask price
        while let (Some(bid_level_node_handle), Some(ask_level_node_handle)) = 
            {
                //let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                (Some(order_book_handle.clone().lock_unwrap().best_bid()), Some(order_book_handle.clone().lock_unwrap().best_ask()))
            } 
            {
            // Break the loop if bid price is lower than ask price (no arbitrage opportunity)
            if bid_level_node_handle.clone().lock_unwrap().price < ask_level_node_handle.clone().lock_unwrap().price {
                break;
            }

            // Retrieve the front orders of both bid and ask levels
            let mut bid_order_node_handle = bid_level_node_handle.clone().lock_unwrap().order_list.front().map(|node| node.clone());
            let mut ask_order_node_handle = ask_level_node_handle.clone().lock_unwrap().order_list.front().map(|node| node.clone());

            // Process each pair of bid and ask orders
            while let (Some(bid_node_handle), Some(ask_node_handle)) = (bid_order_node_handle, ask_order_node_handle) {
                let next_bid_order_node_handle = bid_node_handle.clone().lock_unwrap().next_mut();
                let next_ask_order_node_handle = ask_node_handle.clone().lock_unwrap().next_mut();
                // Check for All-Or-None (AON) orders and handle them separately
                if bid_node_handle.clone().lock_unwrap().is_aon() || ask_node_handle.clone().lock_unwrap().is_aon() {
                    // Calculate the matching chain for AON orders
                    let chain = calculate_matching_chain_cross_levels(order_book_handle.clone(), bid_level_node_handle.clone(), ask_level_node_handle.clone());

                    // If no matching chain is found, exit the function
                    if chain == 0 {
                        return;
                    }

                    // Execute matching chains for AON orders
                    if bid_node_handle.clone().lock_unwrap().is_aon() {
                        let price = bid_node_handle.clone().lock_unwrap().price;
                        execute_matching_chain(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), bid_level_node_handle.clone(), price, chain);
                        execute_matching_chain(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), ask_level_node_handle.clone(), price, chain);
                    } else {
                        let price = ask_node_handle.clone().lock_unwrap().price;
                        execute_matching_chain(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), ask_level_node_handle.clone(), price, chain);
                        execute_matching_chain(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), bid_level_node_handle.clone(), price, chain);
                    }
                    break;
                }

                // Determine which order to execute and which to reduce based on leaves quantity
                let (mut executing_order_handle, mut reducing_order_handle) = if bid_node_handle.clone().lock_unwrap().leaves_quantity > ask_node_handle.clone().lock_unwrap().leaves_quantity {
                    (ask_node_handle, bid_node_handle)
                } else {
                    (bid_node_handle, ask_node_handle)
                };
                
                // Determine the quantity and price for execution
                let quantity = executing_order_handle.clone().lock_unwrap().leaves_quantity;
                let price = executing_order_handle.clone().lock_unwrap().price;
                
                // Execute the selected order
                // market_handler.on_execute_order(&executing_order_handle.clone().lock_unwrap().order_handle, price, quantity);
                {
                    let mut order_book_handle_locked = order_book_handle.lock_unwrap();
                    order_book_handle_locked.update_last_price(executing_order_handle.clone().lock_unwrap().order_handle.clone(), price);
                    order_book_handle_locked.update_matching_price(executing_order_handle.clone().lock_unwrap().order_handle.clone(), price);
                }
                
                // Update the executed order's quantity
                executing_order_handle.clone().lock_unwrap().executed_quantity += quantity;
                // Reduce the quantity of the executing order
                delete_order_recursive(orders.clone(), order_book_handles.clone(), order_pool.clone(), market_handler.clone(), executing_order_handle.clone().lock_unwrap().id, true, false);
                
                // Execute the reducing order
                // market_handler.on_execute_order(&reducing_order_handle.clone().lock_unwrap().order_handle, price, quantity);
                //
                {
                   // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                    order_book_handle.clone().lock_unwrap().update_last_price(reducing_order_handle.clone().lock_unwrap().order_handle.clone(), price);
                    order_book_handle.clone().lock_unwrap().update_matching_price(reducing_order_handle.clone().lock_unwrap().order_handle.clone(), price);
                }
                
                // Update the reducing order's quantity
                reducing_order_handle.clone().lock_unwrap().executed_quantity += quantity;

                // Decrease the leaves quantity of the executing order
                executing_order_handle.clone().lock_unwrap().leaves_quantity -= quantity;

                // Move to the next pair of orders at the same level
                bid_order_node_handle = next_bid_order_node_handle;
                ask_order_node_handle = next_ask_order_node_handle;
            }
            // Activate stop orders based on price changes
            // let best_buy_stop: LevelNodeHandle;
            // let market_ask_price: u64;
            // {
            //     let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            //     let best_buy_stop = order_book_handle_locked.best_buy_stop();
            //     let market_ask_price = order_book_handle_locked.get_market_ask_price();
            // }
            
            activate_stop_orders_level(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().best_buy_stop().clone(), order_book_handle.clone().lock_unwrap().get_market_ask_price().clone());
            
            // let best_sell_stop: LevelNodeHandle;
            // let market_bid_price: u64;
            // {
            //     let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            //     let best_sell_stop = order_book_handle_locked.best_sell_stop();
            //     let market_bid_price = order_book_handle_locked.get_market_bid_price();
            // }
            activate_stop_orders_level(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().best_sell_stop().clone(), order_book_handle.clone().lock_unwrap().get_market_bid_price().clone());
        }

        // Keep activating stop orders until no more can be activated
        if !activate_stop_orders(orders.clone(), order_book_handles.clone(), market_handler.clone(), order_pool.clone(), order_book_handle.clone()) {
            break;
        }
    }
}

fn calculate_matching_chain_cross_levels(
    mut order_book_handle: OrderBookHandle,
    bid_level_node_handle: LevelNodeHandle,
    ask_level_node_handle: LevelNodeHandle,
) -> u64 {
    let mut longest_level_handle = bid_level_node_handle.clone();
    let mut shortest_level_handle = ask_level_node_handle.clone();
    let mut longest_order_handle = bid_level_node_handle.clone().lock_unwrap().order_list.front().map(|node| node.clone());
    let mut shortest_order_handle = ask_level_node_handle.clone().lock_unwrap().order_list.front().map(|node| node.clone());
    let mut required = longest_order_handle.clone().unwrap().clone().lock_unwrap().leaves_quantity;
    let mut available = 0;

    // Find the initial longest order chain
    if let (Some(longest), Some(shortest)) = (longest_order_handle.clone(), shortest_order_handle.clone()) {
        if longest.clone().lock_unwrap().is_aon() && shortest.clone().lock_unwrap().is_aon() {
            if shortest.clone().lock_unwrap().leaves_quantity > longest.clone().lock_unwrap().leaves_quantity {
                required = shortest.clone().lock_unwrap().leaves_quantity;
                available = 0;
                std::mem::swap(&mut longest_level_handle, &mut shortest_level_handle);
                std::mem::swap(&mut longest_order_handle, &mut shortest_order_handle);
            }
        } else if shortest.clone().lock_unwrap().is_aon() {
            required = shortest.clone().lock_unwrap().leaves_quantity;
            available = 0;
            std::mem::swap(&mut longest_level_handle, &mut shortest_level_handle);
            std::mem::swap(&mut longest_order_handle.clone(), &mut shortest_order_handle.clone());
        }
    }

    let mut longest_level_handle = Some(longest_level_handle);
    let mut shortest_level_handle = Some(shortest_level_handle);

    // Travel through price levels
    while let (Some(bid_level_node_handle), Some(ask_level_node_handle)) = (longest_level_handle.clone(), shortest_level_handle.clone()) {
        while let (Some(bid_order_handle), Some(ask_order_handle)) = (longest_order_handle.clone(), shortest_order_handle.clone()) {
            let need = required.saturating_sub(available);
            let quantity = if shortest_order_handle.clone().unwrap().clone().lock_unwrap().is_aon() {
                shortest_order_handle.clone().unwrap().clone().lock_unwrap().leaves_quantity
            } else {
                std::cmp::min(shortest_order_handle.clone().unwrap().clone().lock_unwrap().leaves_quantity, need)
            };
            available += quantity;

            if required == available {
                return required;
            }

            if required < available {
                let next = longest_order_handle.unwrap().clone().lock_unwrap().next_mut();
                longest_order_handle = shortest_order_handle;
                shortest_order_handle = next;
                std::mem::swap(&mut required, &mut available);
                continue;
            }
        }

        // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
        order_book_handle.clone().lock_unwrap().get_next_level(bid_level_node_handle);
        //  longest_order_handle = longest_level_handle.and_then(|level| level.lock_unwrap().order_list.front()).map(|node| node.clone());
        let mut longest_order_handle = None;

        if let Some(ref level_handle) = longest_level_handle {
            let level = level_handle.clone().lock_unwrap().clone(); // Lock the level
            if let Some(order_node) = level.front() {
                longest_order_handle = Some(order_node.clone()); // Clone the order node
            }
        }

        // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
        order_book_handle.clone().lock_unwrap().get_next_level(ask_level_node_handle);
        if let Some(ref level_handle) = shortest_level_handle {
            let level = level_handle.clone().lock_unwrap().clone(); // Lock the level
            if let Some(order_node) = level.front() {
                shortest_order_handle = Some(order_node.clone()); // Clone the order node
            }
        }
    }
    0
}

fn match_market(
    order_pool: OrderPool,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    mut order_book_handle: OrderBookHandle,
    market_handler: MarketHandler, 
    orders: HashMap<u64, OrderNodeHandle>,
    mut order_handle: OrderHandle
    ) -> Result<(), ErrorCode> {
    // Calculate acceptable market order price with optional slippage value
    match order_handle.clone().lock_unwrap().order_type {
        OrderType::Buy | OrderType::Market => {
            let best_ask: LevelNodeHandle;
            order_handle.clone().lock_unwrap().price = order_book_handle.clone().lock_unwrap().best_ask.clone().lock_unwrap().price.saturating_add(order_handle.clone().lock_unwrap().slippage);
        },
        _ => {
            let best_bid: LevelNodeHandle;
            order_handle.clone().lock_unwrap().price = order_book_handle.clone().lock_unwrap().best_ask.clone().lock_unwrap().price.saturating_sub(order_handle.clone().lock_unwrap().slippage);
        },
    }

    match_order(order_pool, orders, market_handler, order_book_handles, order_book_handle, order_handle);

    Ok(())
}

fn delete_order(
    mut orders: HashMap<u64, OrderNodeHandle>, 
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler,
    order_pool: OrderPool,
    id: u64, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {
    delete_order_recursive(orders, order_book_handles, order_pool, market_handler, id, true, false)
}

fn delete_order_recursive(
    mut orders: HashMap<u64, OrderNodeHandle>, 
    order_book_handles: HashMap<u64, OrderBookHandle>,
    mut order_pool: OrderPool,
    market_handler: MarketHandler,
    id: u64, 
    matching: bool, 
    recursive: bool
) -> Result<(), ErrorCode> {
    // Validate parameters
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // get the order to delete
    let order_node_handle = orders.get(&id).ok_or(ErrorCode::OrderNotFound)?;

    // get the valid order book for the order
    let order_book_handle = order_book_handles.get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

    // Delete the order from the order book
    match order_node_handle.clone().lock_unwrap().order_type {
        OrderType::Limit => {
           // let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            update_level(market_handler.clone(), order_book_handles.get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone(), 
            order_book_handles.get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_order(order_node_handle.clone()));
        },
        OrderType::Stop | OrderType::StopLimit => {
           // let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handles.get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_stop_order(order_node_handle.clone());
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
           // let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            order_book_handles.get(&order_node_handle.clone().lock_unwrap().symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.clone().lock_unwrap().delete_trailing_stop_order(order_node_handle.clone());
        },
        _ => return Err(ErrorCode::OrderTypeInvalid),
    };

    // Call the corresponding handler
    // market_handler.on_delete_order_node(order_node);

    // Erase the order
    orders.clone().remove(&id);

    // Release the order
    order_pool.release(order_node_handle.clone());

    // Automatic order matching
    if matching && !recursive {
        match_order_book_handle(order_book_handles.clone(), order_pool, order_book_handle.clone(), orders, market_handler.clone());
    }

    order_book_handle.clone().lock_unwrap().reset_matching_price();

    // Reset matching price
    Ok(())
}

fn activate_stop_orders(
    orders: HashMap<u64, OrderNodeHandle>,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler,
    order_pool: OrderPool,
    mut order_book_handle: OrderBookHandle
    ) -> bool {

    let mut result = false;
    let mut stop = false;

    while !stop {
        stop = true;

        // Try to activate buy stop orders
        {
            let best_ask;
            {
                //let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                    if activate_individual_stop_orders(order_book_handles.clone(), order_pool.clone(), orders.clone(), market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().best_buy_stop(), order_book_handle.clone().lock_unwrap().get_market_ask_price())
                        || activate_individual_stop_orders(order_book_handles.clone(), order_pool.clone(), orders.clone(), market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().best_trailing_buy_stop(), order_book_handle.clone().lock_unwrap().get_market_ask_price()) {
                    result = true;
                    stop = false;
                }
                best_ask = order_book_handle.clone().lock_unwrap().best_ask();
            }

            // Recalculate trailing buy stop orders
            recalculate_trailing_stop_price(order_book_handle.clone(), best_ask.clone());

            let best_bid;
            {
              //  let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
                // Try to activate sell stop orders
                if activate_individual_stop_orders(order_book_handles.clone(), order_pool.clone(), orders.clone(), market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().best_sell_stop(), order_book_handle.clone().lock_unwrap().get_market_bid_price())
                    || activate_individual_stop_orders(order_book_handles.clone(), order_pool.clone(), orders.clone(), market_handler.clone(), order_book_handle.clone(), order_book_handle.clone().lock_unwrap().best_trailing_sell_stop(), order_book_handle.clone().lock_unwrap().get_market_bid_price()) {
                    result = true;
                    stop = false;
                }
                best_bid = order_book_handle.clone().lock_unwrap().best_bid();
            }

            // Recalculate trailing sell stop orders
            recalculate_trailing_stop_price(order_book_handle.clone(), best_bid.clone());
        }
    }
    result
}

fn activate_individual_stop_orders(
    order_book_handles: HashMap<u64, OrderBookHandle>,
    order_pool: OrderPool,
    orders: HashMap<u64, OrderNodeHandle>,
    market_handler: MarketHandler,
    order_book_handle: OrderBookHandle, 
    level_node_handle: LevelNodeHandle,
    stop_price: u64
    ) -> bool {

    let mut result = false;

    let arbitrage = if level_node_handle.clone().lock_unwrap().is_bid() {
        stop_price <= level_node_handle.clone().lock_unwrap().price
    } else {
        stop_price >= level_node_handle.clone().lock_unwrap().price
    };
    if !arbitrage {
        return false;
    }

    let mut activating_order_node_handle = level_node_handle.clone().lock_unwrap().order_list.front_mut().map(|node| node.clone());

    while let Some(order_node_handle) = activating_order_node_handle {

        let mut next_activating_order_node_handle = order_node_handle.clone().lock_unwrap().next_mut();

        match order_node_handle.clone().lock_unwrap().order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= activate_stop_order(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), order_node_handle.clone());
            },
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= activate_stop_limit_order(order_book_handles.clone(), order_pool.clone(), order_book_handle.clone(), orders.clone(), market_handler.clone(), order_node_handle.clone());
            },
            _ => panic!("Unsupported order type!"),
        }

        activating_order_node_handle = next_activating_order_node_handle;
    }
    result
}

fn activate_stop_order(
    mut order_pool: OrderPool,
    mut orders: HashMap<u64, OrderNodeHandle>, 
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler,
    mut order_book_handle: OrderBookHandle, 
    mut order_node_handle: OrderNodeHandle
    ) -> bool {
    // Delete the stop order from the order book
    if order_node_handle.clone().lock_unwrap().is_trailing_stop() || order_node_handle.clone().lock_unwrap().is_trailing_stop_limit() {
       // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
        order_book_handle.clone().lock_unwrap().delete_trailing_stop_order(order_node_handle.clone());
    } else {
       // let mut order_book_handle_locked = order_book_handle.lock_unwrap();
        order_book_handle.clone().lock_unwrap().delete_stop_order(order_node_handle.clone());
    }

    // Convert the stop order into the market order
    order_node_handle.clone().lock_unwrap().order_type = OrderType::Market;
    order_node_handle.clone().lock_unwrap().price = 0;
    order_node_handle.clone().lock_unwrap().stop_price = 0;
    order_node_handle.clone().lock_unwrap().time_in_force = if order_node_handle.clone().lock_unwrap().is_fok() { TimeInForce::FOK } else { TimeInForce::IOC };

    // Call the corresponding handler
    // market_handler.on_update_order(&order_node_handle.clone().lock_unwrap().order_handle);

    // Match the market order
    match_market(order_pool.clone(), order_book_handles, order_book_handle, market_handler, orders.clone(), order_node_handle.clone().lock_unwrap().order_handle.clone());

    // Call the corresponding handler
    // market_handler.on_delete_order_node(order_node);

    // Erase the order
    orders.clone().remove(&order_node_handle.clone().lock_unwrap().id);

    // Release the order_handle, assuming we have an order pool with a release method
    order_pool.clone().release(order_node_handle);

    true
}

fn activate_stop_orders_level(
    order_pool: OrderPool,
    orders: HashMap<u64, OrderNodeHandle>,
    order_book_handles: HashMap<u64, OrderBookHandle>,
    market_handler: MarketHandler,
    order_book_handle: OrderBookHandle, 
    level_node_handle: LevelNodeHandle, 
    stop_price: u64
    ) -> bool {

    let mut result = false;
    
    let arbitrage = if level_node_handle.clone().lock_unwrap().is_bid() {
        stop_price <= level_node_handle.clone().lock_unwrap().price
    } else {
        stop_price >= level_node_handle.clone().lock_unwrap().price
    };

    if !arbitrage {
        return false;
    }

    let mut activating_order_handle = level_node_handle.clone().lock_unwrap().order_list.front().cloned();
    while let Some(order_node_handle) = activating_order_handle {
        // Clone next_order to avoid borrow_muting issues
        let next_activating_order_handle = order_node_handle.clone().lock_unwrap().next_mut().as_mut().cloned();

        match order_node_handle.clone().lock_unwrap().order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= activate_stop_order(order_pool.clone(), orders.clone(), order_book_handles.clone(), market_handler.clone(), order_book_handle.clone(), order_node_handle.clone());
            }
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= activate_stop_limit_order(order_book_handles.clone(), order_pool.clone(), order_book_handle.clone(), orders.clone(), market_handler.clone(), order_node_handle.clone());
            }
            _ => {
                assert!(false, "Unsupported order type!");
            }
        }
        //let next_order_handle = next_activating_order_handle.clone();
        activating_order_handle = next_activating_order_handle;
    }
    true
}

fn recalculate_trailing_stop_price(mut order_book_handle: OrderBookHandle, level_node_handle: LevelNodeHandle) {
    let mut new_trailing_price;

    // Skip recalculation if market price goes in the wrong direction
    match level_node_handle.clone().lock_unwrap().level_type {
        Some(LevelType::Ask) => {
            {
                let mut order_book_handle_locked = order_book_handle.lock_unwrap();
                let old_trailing_price = order_book_handle_locked.trailing_ask_price;
                new_trailing_price = order_book_handle_locked.get_market_trailing_stop_price_ask();
                if new_trailing_price >= old_trailing_price {
                    return;
                }
                order_book_handle_locked.trailing_ask_price = new_trailing_price;
            }
        },
        Some(LevelType::Bid) => {
            {
                let mut order_book_handle_locked = order_book_handle.lock_unwrap();
                let old_trailing_price = order_book_handle_locked.trailing_bid_price;
                new_trailing_price = order_book_handle_locked.get_market_trailing_stop_price_bid();
                if new_trailing_price <= old_trailing_price {
                    return;
                }
                order_book_handle_locked.trailing_bid_price = new_trailing_price;
            }
        },
        None => todo!(),
    }

    // Recalculate trailing stop orders
    let mut current = match level_node_handle.clone().lock_unwrap().level_type.clone().unwrap() {
        LevelType::Ask => {
          //  let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            order_book_handle.clone().lock_unwrap().best_trailing_buy_stop.clone()
        },
        LevelType::Bid => {
           // let mut order_book_handle_locked = order_book_handle.clone().lock_unwrap();
            order_book_handle.clone().lock_unwrap().best_trailing_sell_stop.clone()
        }
    };

    let mut previous: Option<LevelNodeHandle> = None;
    let mut current = Some(current);
    while let Some(ref current_level) = current {
        let mut recalculated = false;
        let mut node = current_level.lock_unwrap().order_list.front_mut().map(|node| node.clone());

        while let Some(order_node_handle) = node {
            let next_order = order_node_handle.clone().lock_unwrap().next_mut();
            let old_stop_price = order_node_handle.clone().lock_unwrap().stop_price;
            let new_stop_price;
            {
                let mut order_book_handle_locked = order_book_handle.lock_unwrap();
                new_stop_price = order_book_handle_locked.calculate_trailing_stop_price(order_node_handle.clone().lock_unwrap().order_handle.clone());
            }

            // Update and re-add order if stop price changed
            if new_stop_price != old_stop_price {
                {
                    let mut order_book_handle_locked = order_book_handle.lock_unwrap();
                    order_book_handle_locked.delete_trailing_stop_order(order_node_handle.clone());
                }
                // Update stop price based on order type
                match order_node_handle.clone().lock_unwrap().order_type {
                    OrderType::TrailingStop => order_node_handle.clone().lock_unwrap().stop_price = new_stop_price,
                    OrderType::TrailingStopLimit => {
                        let diff = order_node_handle.clone().lock_unwrap().price - order_node_handle.clone().lock_unwrap().stop_price;
                        order_node_handle.clone().lock_unwrap().stop_price = new_stop_price;
                        order_node_handle.clone().lock_unwrap().price = new_stop_price + diff;
                    },
                    _ => panic!("Unsupported order type!"),
                }
                // market_handler.on_update_order(&order_node_handle.clone().lock_unwrap().order_handle);

                {
                    let mut order_book_handle_locked = order_book_handle.lock_unwrap();
                    order_book_handle_locked.add_trailing_stop_order(order_node_handle.clone());
                }
                recalculated = true;
            }
            node = next_order;
        }

        if recalculated {
            let current = if let Some(ref prev) = previous {
                Some(prev.clone()) 
            } else if level_node_handle.clone().lock_unwrap().level_type == Some(LevelType::Ask) {
                Some(order_book_handle.lock_unwrap().best_trailing_buy_stop.clone())
            } else {
                Some(order_book_handle.lock_unwrap().best_trailing_sell_stop.clone())
            };
        } else {
            previous = current.clone();
            let mut order_book_handle_locked = order_book_handle.lock_unwrap();
            current = Some(order_book_handle_locked.get_next_trailing_stop_level(current_level.clone()));
        }
    }
}

fn update_level(market_handler: MarketHandler, order_book_handle: OrderBookHandle, update: LevelUpdate) -> Result<(), &'static str> {
    match update.update_type {
        UpdateType::Add => market_handler.on_add_level(order_book_handle.clone(), &update.update, update.top),
        UpdateType::Update => market_handler.on_update_level(order_book_handle.clone(), &update.update, update.top),
        UpdateType::Delete => market_handler.on_delete_level(order_book_handle.clone(), &update.update, update.top),
        _ => {
            eprintln!("Warning: Received an unexpected update type in update_level");
        },
    };
    Ok(market_handler.on_update_order_book(order_book_handle.clone(), update.top))
}

