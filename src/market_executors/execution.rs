
use std::{cell::RefCell, cmp::min, marker::PhantomData, rc::Rc};

use crate::{levels::{indexing::LevelNode, level::{Level, LevelUpdate, UpdateType}}, market_handler::Handler, order_book::order_book::OrderBook, orders::{order::{ErrorCode, Order, OrderType, TimeInForce}, orders::{OrderOps, Orders}}};

use super::{executor::Execution, order_book_operations::{OBMap, OrderBookContainer}};

pub fn add_order<E>(orders: &Orders, order_books: OBMap, order: &Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>
where
    E: Execution + Handler + OrderOps,
{
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
            E::add_market_order(order_books, order, matching, recursive)
        },
        OrderType::Limit => {
            E::add_limit_order(order, matching, order_books, recursive)
        },
        OrderType::Stop | OrderType::TrailingStop => {
            E::add_stop_order(orders, order_books, order, matching, recursive)
        },
        OrderType::StopLimit | OrderType::TrailingStopLimit => {
            E::add_stop_limit_order(order_books, orders, order, matching, recursive)
        },
        _ => Err(ErrorCode::OrderTypeInvalid),
    }
}

pub fn update_level<E>(order_book: &OrderBook, update: &LevelUpdate)    
where 
    E: Execution + Handler + OrderOps
{
    match update.update_type {
        UpdateType::Add => E::on_add_level(order_book, &update.update, update.top),
        UpdateType::Update => E::on_update_level(order_book, &update.update, update.top),
        UpdateType::Delete => E::on_delete_level(order_book, &update.update, update.top),
        _ => return,
    };
    E::on_update_order_book(order_book, update.top)
}

pub fn add_market_order<E>(mut order_books: OBMap, order: &mut Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where                          
    E: Execution + Handler + OrderOps,
{
    let mut order_book = order_books.get_order_book(&order.symbol_id).expect("order book not found");

    // let some_condition = true;
    // if some_condition {
    //     matching = true;
    //     recursive = false;
    // }

    E::on_add_order(&order);

    if matching && !recursive {
        E::match_market(order);
    }

    E::on_delete_order(&order);

    if matching && !recursive {
        E::match_order_book(); // Assuming match_order also returns a Result
    }
    
    order_book.reset_matching_price();

    Ok(())
}

pub fn execute_matching_chain<E>(orders: &Orders, order_book: &mut OrderBook, mut level_node: Option<Rc<RefCell<LevelNode>>>, price: u64, mut volume: u64) -> Result<(), E>
where
    E: Execution + Handler + OrderOps,
{
    while volume > 0 {
        if let Some(current_level) = level_node.take() { // Take the current level to consume it.
            let mut executing_order = match current_level.try_borrow_mut() {
                Ok(mut level_ref) => level_ref.level.orders.front_mut(),
                Err(_) => {
                    // Handle the error case, e.g., by logging or using a fallback
                    // Returning `None` here for simplicity
                    None
                },
            };

            while volume > 0 {
                if let Some(order) = executing_order.take() { // Take the executing order to consume it.
                    let quantity = if order.is_aon() {
                        order.leaves_quantity
                    } else {
                        std::cmp::min(order.leaves_quantity, volume)
                    };

                    E::on_execute_order(&order, price, quantity);
                    order_book
                        .update_last_price(order, price);
                    order_book
                        .update_matching_price(order, price);
                    
                    order.executed_quantity += quantity;
                    E::reduce_order(orders, order.id, quantity, true, false);
                    volume -= quantity;
                    executing_order = order.next_mut();
                } else {
                    break;
                }
            }
            level_node = order_book.get_next_level_node(current_level.clone()).map_err(|e| e);
        } else {
            break;
        }
    }
    Ok(())
}

pub fn add_limit_order<E>(orders: &mut Orders, order: &Order, matching: bool, order_books: &mut OBMap, recursive: bool) -> Result<(), ErrorCode> 
where                                         
    E: Execution + Handler + OrderOps,
{     
    let mut order_book = order_books
        .get_order_book(&order.symbol_id)
        .expect("order book not found");

    E::on_add_order(&order);

    if matching && !recursive {
        E::match_limit(&order);
    }

    if (order.leaves_quantity > 0) && !order.is_ioc() && !order.is_fok() {
    // let order = order.new(&order);
        if orders.insert_order(&order.id, order).is_some() {
            // Order duplicate
            E::on_delete_order(&order);
            // order_pool.release(order.new(&order));
        } else {
            // Update level with the new order
            // let order_book = E::add_order(order.new(&order));
            let level_update = order_book.add_order(order);
            E::update_level(order_book, level_update);
        }
    } else {
        E::on_delete_unmatched_order(&order);
    }

    if matching && !recursive {
        E::match_order_book();
    }

    order_book.reset_matching_price();

    Ok(())
}




    fn match_limit(order: &Order) 
    {
        // Match the limit order
        E::match_order(order);
    }

    fn match_market(order_book: &mut OrderBook, order: &mut Order) -> Result<(), ErrorCode> 
    {
        // Calculate acceptable market order price with optional slippage value
        match order.order_type {
            OrderType::Buy | OrderType::Market => {
                order.price = (order_book.best_ask().expect("best ask not retrieved")).try_borrow_mut().level.price.saturating_add(order.slippage);
            },
            _ => {
                order.price = (order_book.best_bid().expect("best bid not retrieved")).try_borrow_mut().level.price.saturating_sub(order.slippage);
            },
        }

        E::match_order(order);

        Ok(())
    }

fn match_order<E>(order_book: &mut OrderBook, order: &mut Order) -> Result<(), ErrorCode>
where
    E: Execution + Handler + OrderOps,
{
    let level_node = if order.is_buy() {
        order_book.best_ask().ok_or(ErrorCode::DefaultError)?
    } else {
        order_book.best_bid().ok_or(ErrorCode::DefaultError)?
    };

    let arbitrage = if order.is_buy() {
        order.price >= level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.price
    } else {
        order.price <= level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.price
    };

    if !arbitrage {
        return Ok(());
    }
    // Special case for 'Fill-Or-Kill'/ll-Or-None' order
    if order.is_fok() || order.is_aon() {
        let chain = E::calculate_matching_chain_single_level(level_node.clone(), order.price, order.leaves_quantity);

        E::execute_matching_chain(level_node, order.price, chain);

        E::on_execute_order(&order, order.price, order.leaves_quantity);
        
        order_book.update_last_price(order, order.price);
        order_book.update_matching_price(order, order.price);
        
        order.executed_quantity += order.leaves_quantity;
        order.leaves_quantity = 0;

        return Ok(());
    }

    let mut binding = level_node.as_ref().expect("node needed").try_borrow_mut();
    let mut executing_order = binding.level.orders.front_mut();

    // Execute crossed orders
    while let Some(order) = executing_order {

        // get the execution quantity
        let quantity = order.leaves_quantity.min(order.leaves_quantity);

        // Special case for ll-Or-None' orders
        if order.is_aon() && (order.leaves_quantity > order.leaves_quantity) {
            return Ok(());
        }

        // get the execution price
        let price = order.price;

        // Call the corresponding MarketHandler
        E::on_execute_order(&order, quantity, price);

        // Update the corresponding market price
        order_book.update_matching_price(order, order.price);

        // Increase the order executed quantity
        order.executed_quantity += quantity;

        // Reduce the executing order in the order book
        // orders implementation
        E::reduce_order(&Orders::default(), order.symbol_id, quantity, false, false);
        
        // Call the corresponding MarketHandler
        E::on_execute_order(&order, price, quantity);

        // Update the corresponding market price
        order_book.update_last_price(order, order.price);
        order_book.update_matching_price(order, order.price);

        // Increase the order executed quantity
        order.executed_quantity += quantity;

        // Reduce the order leaves quantity
        order.leaves_quantity -= quantity;
        if order.leaves_quantity == 0 {
            return Ok(());
        }
        
        let next_executing_order = order.next_mut();
        
        // Move to the next order to execute at the same price level
        if let Some(node) = next_executing_order {
            executing_order = Some(node);
        } else {
            break;
        }
    }   

    // Continue with the rest of the logic, replacing expect and unwraps with proper error handling...

    Ok(())
}

fn match_order_book(&mut self) -> Result<(), ErrorCode>
{
    loop {
        let (bid_level_node, ask_level_node) = match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => (bid, ask),
            // Assuming no error if there's simply nothing to match
            _ => return Ok(()),
        };

        let (mut bid_level, mut ask_level) = (
            bid_level_node.try_borrow_mut().map_err(|_| ErrorCode::DefaultError)?,
            ask_level_node.try_borrow_mut().map_err(|_| ErrorCode::DefaultError)?,
        );

        if bid_level.level.price < ask_level.level.price {
            break;
        }

        // Retrieve the front E::orders of both bid and ask levels
        let mut bid_order = bid_level.level.orders.front_mut();
        let mut ask_order = ask_level.level.orders.front_mut();

        // Process each pair of bid and ask orders
        while let (Some(bid_order_handle), Some(ask_order_handle)) = (bid_order, ask_order) {
                
            // Check for All-Or-None (AON) E::orders and handle them separately
            if bid_order_handle.is_aon() || ask_order_handle.is_aon() {
                // Calculate the matching chain for AON E::orders
                let chain = E::calculate_matching_chain_cross_levels(Some(bid_level_node.clone()), Some(ask_level_node.clone()));

                // If no matching chain is found, exit the function
                if chain == 0 {
                    return Ok(());
                }

                // Execute matching chains for AON E::orders
                if bid_order_handle.is_aon() {
                    let price = bid_order_handle.level_node.as_ref().expect("bid order handle node not retrieved").try_borrow().level.price;
                    E::execute_matching_chain(Some(bid_level_node.clone()), price, chain);
                    E::execute_matching_chain(Some(ask_level_node.clone()), price, chain);
                } else {
                    let price = ask_order_handle.level_node.as_ref().expect("ask order handle node not retrieved").try_borrow().level.price;
                    E::execute_matching_chain(Some(ask_level_node.clone()), price, chain);
                    E::execute_matching_chain(Some(bid_level_node.clone()), price, chain);
                }
                break;
            }

            // Determine which order to execute and which to reduce based on leaves quantity
            let (mut executing_order, mut reducing_order) = if bid_order_handle.leaves_quantity > ask_order_handle.leaves_quantity {
                (ask_order_handle.clone(), bid_order_handle.clone())
            } else {
                (bid_order_handle.clone(), ask_order_handle.clone())
            };
            
            // Determine the quantity and price for execution
            let quantity = executing_order.leaves_quantity;
            let price = executing_order.price;
            
            // Execute the selected order
            E::on_execute_order(&executing_order, price, quantity);
            self.update_last_price(&executing_order, price);
            self.update_matching_price(&executing_order, price);
            
            // Update the executed order's quantity
            executing_order.executed_quantity += quantity;
            // Reduce the quantity of the executing order
            <OrderBook as Matching<E>>::delete_order_recursive(self, executing_order.id, true, false, OBMap::default(), &mut Orders::default());
            
            // Execute the reducing order
            E::on_execute_order(&reducing_order, price, quantity);
            
            self.update_last_price(&reducing_order, price);
            self.update_matching_price(&reducing_order, price);
            
            // Update the reducing order's quantity
            reducing_order.executed_quantity += quantity;

            // Decrease the leaves quantity of the executing order
            executing_order.leaves_quantity -= quantity;

            // Move to the next pair of E::orders at the same level
            bid_order = bid_order_handle.next_mut();
            ask_order = ask_order_handle.next_mut();
        }
        
        E::activate_stop_orders_level(self.best_buy_stop(), self.get_market_ask_price());
        E::activate_stop_orders_level(self.best_sell_stop(), self.get_market_bid_price());
    }

    // Keep activating stop E::orders until no more can be activated
    if !E::activate_stop_orders() {
        return Ok(());
    }
    Ok(())
}


fn delete_order_recursive(&mut self, id: u64, matching: bool, recursive: bool, mut order_books: OBMap, orders: &mut Orders) -> Result<(), ErrorCode>
{
    // Validate parameters
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // get the order to delete
    let order = orders.get_order(id)?;

    // get the valid order book for the order
    // use error code possibly
    let order_book: &mut OrderBook = order_books.get_mut(&order.id).ok_or(ErrorCode::OrderBookNotFound).expect("order book");

    // Delete the order from the order book
    match order.order_type {
        OrderType::Limit => {
            let level_update = order_book.add_order(order);
            E::update_level(order_book, level_update);
        },
        OrderType::Stop | OrderType::StopLimit => {
            E::delete_stop_order(&order);
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            self.delete_trailing_stop_order(&order);
        },
        _ => return Err(ErrorCode::OrderTypeInvalid),
    };

    // Call the corresponding MarketHandler
    E::on_delete_order(&order);

    // Erase the order
    orders.remove_order(&id);

    // Release the order
    // order_pool.release(order);

    // Automatic order matching
    if matching && !recursive {
        E::match_order_book();
    }

    order_book.reset_matching_price();

    // Reset matching price
    Ok(())
}


fn calculate_matching_chain_single_level<E>(order_book: &OrderBook, mut level_node: Option<Rc<RefCell<LevelNode>>>, price: u64, volume: u64) -> Result<u64, ErrorCode>
where
    E: Execution + Handler + OrderOps,
{
    let mut available = 0;
    // avoid panics later
   // let mut level_node = level_node.expect("rc node failed");
    let mut level_clone = level_node.clone();
    let binding = level_node.as_mut().expect("rc node failed").try_borrow_mut();
    let mut order = binding.level.orders.front();

    while let Some(nodal_level) = level_clone {

        let holder_clone = nodal_level.clone();
        let level = &holder_clone.try_borrow_mut().level;
        // Check the arbitrage bid/ask prices
        let arbitrage = if level.is_bid() {
            price <= level.price
        } else {
            price >= level.price
        };

        if !arbitrage {
            return Ok(0);
        }
        
        // Travel through orders at current price levels
        while let Some(node) = order {

            let need = volume - available;

            let quantity = if node.is_aon() {
                node.leaves_quantity
            } else {
                std::cmp::min(node.leaves_quantity, need)
            };
            available += quantity;

            // Matching is possible, Aeturn the chain size
            if volume == available {
                return available;
            }

            // Matching is not possible
            if volume < available {
                return Ok(0);
            }

            // // Now you can safely call `next_mut()` on the locked node
            let next_node = node.next();

            if let Some(next_order_handle) = next_node {
                order = Some(next_order_handle);
            } else {
                break;
            }
        }

        // Switch to the next price level
        if let Some(next_nodal_level) = order_book.get_next_level_node(nodal_level) {
            // let level_borrow = nodal_level;
            level_clone = Some(next_nodal_level);
        } else {
            break;
        }
    }
    // Matching is not available
    Ok(0)
}

fn calculate_matching_chain_cross_levels<E>(order_book: &mut OrderBook, bid_level: Rc<RefCell<LevelNode>>, ask_level: Rc<RefCell<LevelNode>>) -> Result<u64, ErrorCode>
where
    E: Execution + Handler + OrderOps,
{
    let mut longest_node_level = bid_level.try_borrow_mut();
    let mut shortest_node_level = ask_level.try_borrow_mut();
    // avoid panic
    let mut longest_order = longest_node_level.level.orders.front();
    let mut shortest_order = shortest_node_level.level.orders.front();
    let mut required = longest_order.expect("longest order not found").leaves_quantity;
    let mut available = 0;

    // Find the initial longest order chain
    if let (Some(longest), Some(shortest)) = (longest_order, shortest_order) {
        if longest.is_aon() && shortest.is_aon() {
            if shortest.leaves_quantity > longest.leaves_quantity {
                required = shortest.leaves_quantity;
                available = 0;
                std::mem::swap(&mut longest_order, &mut shortest_order);
                std::mem::swap(&mut longest_node_level, &mut shortest_node_level);
            }
        } else if shortest.is_aon() {
            required = shortest.leaves_quantity;
            available = 0;
            std::mem::swap(&mut longest_order, &mut shortest_order);
            std::mem::swap(&mut longest_node_level, &mut shortest_node_level);
        }
    }

    let b_level = bid_level.clone();
    let a_level = ask_level.clone();
    
    let longest_node_level = b_level.try_borrow();
    let shortest_node_level = a_level.try_borrow();

    let mut longest_order = longest_node_level.level.orders.front();
    let mut shortest_order = shortest_node_level.level.orders.front();

    let mut longest_node_level = Some(bid_level.clone());
    let mut shortest_node_level = Some(ask_level.clone());

    // Travel through price levels
    while let (Some(bid_level), Some(ask_level)) = (longest_node_level.clone(), shortest_node_level.clone()) {
       // let (bid_level, ask_level) = (bid_level.try_borrow_mut(), ask_level.try_borrow_mut());
        while let (Some(bid_order), Some(ask_order)) = (longest_order, shortest_order) {

            let need = required.saturating_sub(available);
            let quantity = if ask_order.is_aon() {
                ask_order.leaves_quantity
            } else {
                std::cmp::min(ask_order.leaves_quantity, need)
            };
            available += quantity;

            // Matching is possible, return the chain size
            if required == available {
                return required;
            }
            
            if required < available {
                // avoid panics in the future
                let next = longest_order.expect("longest order not found").next();
                longest_order = shortest_order;
                shortest_order = next;
                std::mem::swap(&mut required, &mut available);
                continue;
            }
            // Take the next order
            shortest_order = shortest_order.expect("shortest order pointer should output").next()
        }

        if let None = longest_order {
            longest_node_level = order_book.get_next_level_node(bid_level);
            if let Some(ref node_level) = longest_node_level {
                longest_order = node_level.try_borrow_mut().level.orders.pop_front().as_ref();
            };
        }

        if let None = shortest_order {
            shortest_node_level = order_book.get_next_level_node(ask_level);
            if let Some(ref node_level) = shortest_node_level {
                shortest_order = node_level.try_borrow_mut().level.orders.pop_front().as_ref(); // Clone the order node
            }
        }
    }
    Ok(0)
}

pub fn add_stop_order<E>(orders: &mut Orders, mut order_books: OBMap, mut order: &mut Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    // remove panicking behavior from code
    let mut order_book = order_books.get_order_book(&order.symbol_id).expect("order book not found");

    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        order.stop_price = order_book.calculate_trailing_stop_price(order);
    }

    E::on_add_order(&order);

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
                TimeInForce::IOD
            };

            E::on_update_order(&order);
            E::match_market(order);
            E::on_delete_order(&order);
            if matching && !recursive {
                E::match_order_book();
            }
            
            order_book.reset_matching_price();

            return Ok(());
        }
    }

    if order.leaves_quantity > 0 {
        //let mut order = Order::new(order);
        if orders.insert_order(&order.id, order).is_some() {
            // Order duplicate
            E::on_delete_order(&order);
            //order_pool.release(order);
        }
    
        // Add the new stop order into the order book
        if order.is_trailing_stop() || order.is_trailing_stop_limit() {
            order_book.add_trailing_stop_order(&order)
        } else {
            order_book.add_stop_order(&order)
        }
    } else {
        E::on_delete_order(&order);
    }

    if matching && !recursive {
        E::match_order_book();
    }
    
    order_book.reset_matching_price();

    Ok(())
}

pub fn add_stop_limit_order<E>(mut order_books: OBMap, orders: &mut Orders, mut order: &mut Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    // get the valid order book for the order
    let mut order_book = order_books.get_order_book(&order.id).expect("order book not found");

    // Recalculate stop price for trailing stop E::orders
    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        let diff = order.price as u64 - order.stop_price as u64;
        let mut level_update = order_book.calculate_trailing_stop_price(order);
        order.price = (order.stop_price as u64 + diff) as u64;
    }

    // Call the corresponding MarketHandler
    E::on_add_order(&order);

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
            E::on_update_order(&order);

            // Match the limit order
            E::match_limit(&order);

            // Add a new limit order or delete remaining part in case of 'Immediate-Or-Cancel'/'Fill-Or-Kill' order
            if order.leaves_quantity > 0 && !order.is_ioc() && !order.is_fok() {
                // Create a new order
                let mut order = Order::new(order);
                if orders.insert_order(&order.id, &order).is_some() {
                    E::on_delete_order(&order);
                    // order_pool.release(order);
                    // Handle duplicate order case here, if needed
                } else {
                    let update = order_book.add_order(&order);
                    E::update_level(order_book, update);
                }
            } else {
                // Call the corresponding MarketHandler
                E::on_delete_order(&order);
            }

            // Automatic order matching
            if matching && !recursive {
                E::match_order_book();
            }
            order_book.reset_matching_price();
        }
    }

    // Add a new stop order
    if order.leaves_quantity > 0 {
        // Insert the order
        let mut order = Order::new(order);
        if orders.insert_order(&order.id, &order).is_some() {
            // Order duplicate
            E::on_delete_order(&order);
            // order_pool.release(// order.new(&Order));
        }
        // Add the new stop order into the order book
        if order.is_trailing_stop() || order.is_trailing_stop_limit() {
            order_book.add_trailing_stop_order(&order);
        } else {
            order_book.add_stop_order(&order);
        }
    } else {
        // Call the corresponding MarketHandler
        E::on_delete_order(&order);
    }

    // Automatic order matching
    if matching && !recursive {
        E::match_order_book();
    }

    order_book.reset_matching_price();

    Ok(())
}


pub fn execute_order<E>(orders: &mut Orders, order_book: &mut OrderBook, mut order_books: OBMap, id: u64, price: u64, quantity: u64, matching: bool) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    let mut order = orders.get_mut_order(id).map_err(|_| ErrorCode::OrderNotFound)?;

    let mut order_book = order_books.get_order_book(&order.symbol_id).expect("order book not received");

    let quantity = std::cmp::min(quantity, order.leaves_quantity);
    E::on_execute_order(&order, order.price, quantity);
    order_book.update_last_price(order, order.price);
    order_book.update_matching_price(order, order.price);

    let hidden = order.hidden_quantity();
    let visible = order.visible_quantity();
    order.executed_quantity += quantity;
    order.leaves_quantity -= quantity;

    let hidden_delta = hidden - order.hidden_quantity();
    let visible_delta = visible - order.visible_quantity();

    match order.order_type {
        OrderType::Limit => {
            let update = order_book.reduce_order(order, quantity, hidden, visible);
            E::update_level(order_book, update);
        },
        OrderType::Stop | OrderType::StopLimit => { 
            order_book.reduce_stop_order(order, quantity, hidden_delta, visible_delta);
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            E::reduce_trailing_stop_order(&order, quantity, hidden_delta, visible_delta);
        },
        OrderType::Buy => todo!(),
        OrderType::Market => todo!(),
    }

    if order.leaves_quantity > 0 {
        E::on_update_order(&order);
    } else {
        E::on_delete_order(&order);
        orders.remove_order(&id);
        // order_pool.release(orders.get_mut(&id).ok_or(ErrorCode::OrderNotFound)?);
    }

    if matching {
        E::match_order_book();
    }
    order_book.reset_matching_price();

    Ok(())
}

pub fn mitigate_order<E>(id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    E::modify_order(id, new_price, new_quantity, true, true, false)
}

pub fn replace_order_id<E>(symbols: Vec<u64>,id: u64, new_id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    E::replace_order_internal(id, new_id, new_price, new_quantity, true, false)
}

pub fn modify_order<E>(mut orders: &mut Orders, id: u64, new_price: u64, new_quantity: u64, mitigate: bool, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if new_quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }

    // Retrieve and modify the order
    // handle with errorcode going forward
    let mut order = orders.get_mut_order(id)?;
    
    if order.order_type != OrderType::Limit {
        return Err(ErrorCode::OrderTypeInvalid);
    }

    // Apply the modifications
    order.price = new_price;
    order.quantity = new_quantity;
    order.leaves_quantity = new_quantity;

    // In-Flight Mitigation (IFA) logic
    if mitigate {
        order.leaves_quantity = new_quantity.saturating_sub(order.executed_quantity);
    }

    // Handle the updated order
    if order.leaves_quantity > 0 {
        // Handle the case where the order is still active
        // e.g., E::on_update_order(&Order);
    } else {
        // Handle the case where the order is now fully executed
        // e.g., E::on_delete_order(&Order);
        orders.remove_order(&id);
    }

    // Implement matching logic, if required
    if matching && !recursive {
        // Implement logic for matching orders after modification
        // This might involve interacting with an order book or a matching engine
    }

    Ok(())
}


pub fn modify_order_volumes<E>(orders: &mut Orders, id: u64, quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }

    // Retrieve the order node
    let mut order = orders.get_mut_order(id).expect("order");

    // Since MarketExecutor deals with limit orders, assume it has its way of handling them.
    // Here, we focus on the logic specific to reducing a limit order.

    let quantity_to_reduce = std::cmp::min(quantity, order.leaves_quantity);
    order.leaves_quantity -= quantity_to_reduce;

    if order.leaves_quantity > 0 {
        // Handle the case where the order is partially filled
        E::on_update_order(&order);
        // Any additional logic for updating the order goes here
    } else {
        // Handle the case where the order is fully executed
        E::on_delete_order(&order);
        orders.remove_order(&id); // Remove the order from the collection
        // Any additional logic for removing the order goes here
    }

    // Matching logic, if required
    if matching && !recursive {
        // Implement the logic for matching orders after reduction
        // This might involve interacting with an order book or a matching engine
    }

    Ok(())
}

// different from ref impl
pub fn reduce_order<E>(order_books: &mut OBMap, mut order: &mut Order, id: u64, quantity: u64, matching: bool, recursive: bool, orders: &mut Orders) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }
    let mut order_book = order_books.get_order_book(&id).expect("order book not found");
    // let mut order = E::get_order(id);

    let quantity = min(quantity, order.leaves_quantity);
    order.leaves_quantity -= quantity;

    let hidden = order.hidden_quantity();
    let visible = order.visible_quantity();

    // Update the order or delete the empty order
    if order.leaves_quantity > 0 {
        E::on_update_order(&order);
        //let order = order.new(&order);

        // market order book into full order book
        match order.order_type {
            OrderType::Limit => {
                E::reduce_trailing_stop_order(&order, quantity, hidden, visible);
            },
            OrderType::Stop | OrderType::StopLimit => {
                E::reduce_trailing_stop_order(&order, quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                E::reduce_trailing_stop_order(&order, quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        };
    } else {
        E::on_delete_order(&order);
        match order.order_type {
            OrderType::Limit => {
                let update = order_book.reduce_order(order, quantity, hidden, visible);
                E::update_level(order_book, update);
            },
            OrderType::Stop | OrderType::StopLimit => {
                order_book.reduce_stop_order(&order, quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                E::reduce_trailing_stop_order(&order, quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        }

        // Erase the order
        orders.remove_order(&id);
        // Release the Order, assuming we have an order pool with a release method
        // order_pool.release(order);
    }

    if matching && !recursive {
        E::match_order_book();
    }
    
    order_book.reset_matching_price();
    
    Ok(())
}

pub fn replace_order<E>(mut order_books: OBMap, orders: &Orders, id: u64, order: &mut Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    // Delete the previous order by Id
    let order_book = order_books.get_mut(&id).expect("order book not found");
    let mut order = orders.get_mut(&id).expect("order not found");
    order_book.delete_order(order);

    order_book.add_order(order);
    Ok(())
}

pub fn replace_order_internal<E>(id: u64, new_id: u64, new_price: u64, new_quantity: u64, matching: bool, recursive: bool, orders: &mut Orders, mut order_books: OBMap) -> Result<(), ErrorCode> 
where
    E: Execution + Handler + OrderOps,
{
    // Validate parameters 
    if id == 0 || new_id == 0 || new_quantity == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // Retrieve the order to replace
    let order = orders.get_mut_order(id).expect("order not found");
    if !order.is_trailing_stop() && !order.is_trailing_stop_limit() {
        return Err(ErrorCode::OrderTypeInvalid);
    }

    // Retrieve the valid order book for the order
    let mut order_book = order_books.get_order_book(&order.id)?;

    // Delete the trailing stop order from the order book
    order_book.delete_trailing_stop_order(order);

    let new_order = Order {
        id: new_id,
        price: new_price,
        quantity: new_quantity,
        executed_quantity: 0,
        leaves_quantity: new_quantity,
        level_node: order.level_node.clone(),
        ..*order // Clone other fields from the existing order
    };

    // Insert the new order into the manager's collection
    
    if orders.insert_order(&new_id, &new_order).is_some() {
        return Err(ErrorCode::OrderDuplicate);
    }

    // Add the new order into the order book
    order_book.add_trailing_stop_order(&orders[&new_id]);

    // Handle automatic order matching if required
    if matching && !recursive {
        E::match_order_book();
    }

    // Reset matching price in the order book
    order_book.reset_matching_price();

    Ok(())
}

pub fn activate_stop_orders_level<E>(order_book: &mut OrderBook, mut level: &mut Level, stop_price: u64, orders: &Orders) -> bool
where                                         
    E: Execution + Handler + OrderOps,
{
    let mut result = false;
    
    let arbitrage = if level.is_bid() {
        stop_price <= level.price
    } else {
        stop_price >= level.price
    };

    if !arbitrage {
        return Ok(false);
    }

    let mut activating_order = level.orders.front_mut();

    while let Some(order) = activating_order {

        match order.order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= E::activate_stop_order(order_book, orders, order);
            }
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= E::activate_stop_limit_order(order_book, orders, order);
            }
            _ => {
                assert!(false, "Unsupported order type!");
            }
        }
        
        let next_activating_order = order.next_mut();
        activating_order = next_activating_order;
    }
    Ok(result)
}

pub fn activate_stop_orders<E>(order_book: &OrderBook, mut orders: &Orders) -> bool
where                                         
    E: Execution + Handler + OrderOps,
{
    let mut result = false;
    let mut stop = false;

    while !stop {
        stop = true;

        // Try to activate buy stop E::orders
        if E::activate_individual_stop_orders(order_book.best_buy_stop(), order_book.get_market_ask_price(), orders)
            || E::activate_individual_stop_orders(order_book.best_trailing_buy_stop(), order_book.get_market_ask_price(), orders) {
            result = true;
            stop = false;
        }
        let best_ask = order_book.best_ask();
        
        // Recalculate trailing buy stop E::orders
        E::recalculate_trailing_stop_price(best_ask);

        // Try to activate sell stop E::orders
        if E::activate_individual_stop_orders(order_book.best_sell_stop(), order_book.get_market_bid_price(), orders)
            || E::activate_individual_stop_orders(order_book.best_trailing_sell_stop(), order_book.get_market_bid_price(), orders) {
            result = true;
            stop = false;
        }

        let best_bid = order_book.best_bid();
        // Recalculate trailing sell stop E::orders
        E::recalculate_trailing_stop_price(best_bid);
    
    }
    Ok(result)
}

pub fn activate_individual_stop_orders<E>(order_book: &mut OrderBook, level_node: Option<Rc<RefCell<LevelNode>>>, stop_price: u64, orders: &Orders) -> bool
where                                         
    E: Execution + Handler + OrderOps,
{
    let mut activating_order: Option<&mut Order>;
    let arbitrage: bool;

    level_node.ok_or(ErrorCode::DefaultError)
        .and_then(|l_node| l_node.try_borrow_mut().map_err(|_| ErrorCode::DefaultError))
        .and_then(|mut borrowed_node| Ok({
            let arbitrage = match borrowed_node.level.is_bid() {
                true => stop_price <= borrowed_node.level.price,
                false => stop_price >= borrowed_node.level.price,
            };
            activating_order = borrowed_node.level.orders.front_mut();
        })
        ).expect("node");

    if !arbitrage {
        return false
    }
       
    let mut result: bool;
    while let Some(order) = activating_order {

        match order.order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= E::activate_stop_order(order_book, orders, order);
            },
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= E::activate_stop_limit_order(order_book, orders, order);
            },
            _ => panic!("Unsupported order type!"),
        }
        let next_activating_order = order.next_mut();
        activating_order = next_activating_order;
    }
    result
}

pub fn activate_stop_order<E>(order_book: &mut OrderBook, orders: &mut Orders, mut order: &mut Order) -> bool
where                                         
    E: Execution + Handler + OrderOps,
{
    // Delete the stop order from the order book
    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        order_book.delete_trailing_stop_order(&order);
    } else {
        order_book.delete_stop_order(order);
    }

    // Convert the stop order into the market order
    order.order_type = OrderType::Market;
    order.price = 0;
    order.stop_price = 0;
    order.time_in_force = if order.is_fok() { TimeInForce::FOK } else { TimeInForce::IOD };

    // Call the corresponding MarketHandler
    E::on_update_order(&order);

    // Match the market order
    E::match_market(order);

    // Call the corresponding MarketHandler
    E::on_delete_order(&order);

    // Erase the order
    orders.remove_order(&order.id);

    // Release the order, assuming we have an order pool with a release method
    // order_pool.release(order);
    Ok(true)
}

pub fn activate_stop_limit_order<E>(order_book: &mut OrderBook, orders: &mut Orders, order: &mut Order) -> bool
where                                         
    E: Execution + Handler + OrderOps,
{
    // Delete the stop order from the order book
    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        order_book.delete_trailing_stop_order(&order);
    } else {
        order_book.delete_stop_order(order);
    }

    order.order_type = OrderType::Limit;
    order.stop_price = 0;

    E::on_update_order(&order);

    E::match_limit(&order);

    if order.leaves_quantity > 0 && !order.is_ioc() && !order.is_fok() {
        let level_update = order_book.add_order(order);
        E::update_level(order_book, level_update);
    } else {
        // Call the corresponding MarketHandler
        //E::on_delete_order(&order);
        orders.remove_order(&order.id);
        // order_pool.release(order);
    }
    Ok(true)
}
