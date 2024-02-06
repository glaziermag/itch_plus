

use std::{cmp::min};

use crate::{order_book::order_book::{OrderBook, OrderBookOperations, Mutable}, levels::{indexing::{Tree, AccessContents, MutateContents, Ref, MutableBook}, level::LevelUpdate}, orders::{order::{Order, ErrorCode, TimeInForce, OrderType}, orders::{Orders, OrderOps}}, market_handler::Handler};

use super::order_book_operations::{OrderBookContainer, OBMap};

pub trait Execution<'a, B, H, O, T> 
where
    B: MutableBook<'a>,
    R: Ref<'a>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    T: Tree<'a, R>
{
    fn activate_stop_order(order_book: B, order: &Order<R>) -> bool;
    fn activate_stop_limit_order(order_book: B, order: &mut Order<R>) -> bool;
    fn reduce_order(order_book: B, order_id: u64, quantity: u64, hidden: bool, visible: bool) -> Option<R>;
    fn match_order(order_book: B, order: Order<R>);
    fn calculate_matching_chain_single_level(order_book: B, level_node: Option<R>, price: u64, leaves_quantity: u64) -> u64;
    fn calculate_matching_chain_cross_levels(order_book: B, bid_level_node: Option<R>, ask_level_node: Option<R>) -> u64;
    fn execute_matching_chain(order_book: B, level_node: Option<R>, price: u64, chain: u64);
    fn activate_stop_orders(order_book: B) -> bool;
    fn recalculate_trailing_stop_price(order_book: B, best_ask_or_bid: Option<R>);
    fn activate_individual_stop_orders(order_book: B, stop_level_node: Option<R>, market_price: u64, orders: Orders<'a, B>) -> bool;
    fn match_market(order_book: B, order: &Order<R>);
    fn match_limit(order_book: B, order: &Order<R>);
    fn update_level(order_book: B, level_node: LevelUpdate<R>);
    fn match_order_book(order_book: B);
    fn add_limit_order(order: Order<R>, matching: bool, order_books: OBMap<'a, R, T>,recursive: bool) -> Result<(), ErrorCode>;
    fn add_stop_limit_order(order_books: OBMap<'a, R, T>, orders: Orders<'a, B>, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn update_level_on_reduction(order_book: B, order: Order<R>, quantity: u64, hidden: u64, visible: u64);
    //fn link_order(level_node: Option<R>, order: Order<R>);
    fn reduce_trailing_stop_order<B>(order_book: B, order: &Order<R>, quantity: u64, hidden: u64, visible: u64);
    fn get_order(orders: Orders< B>, id: u64) -> Result<Order<'a, R>, ErrorCode>;
    fn replace_order_internal(id: u64, new_id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool) -> Result<(), ErrorCode>;
    fn modify_order(id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool, flag3: bool) -> Result<(), ErrorCode>;
    fn delete_order_recursive(executing_order_id: u64, flag1: bool, flag2: bool);
    fn activate_stop_orders_level(node: Option<R>, stop_price: u64);
    fn add_stop_order(orders: Orders< B>, order_books: OBMap<'a, R, T>, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn remove_order(orders: Orders< B>, id: u64);
    fn add_market_order(order_books: OBMap<'a, R, T>, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
}

// pub trait Internals<'a, B: MutableBook<'a>,>: Execution<'a, B, H, O, T> {
//     fn get_order(orders: Orders< B>, id: u64) -> Result<Order<'a, R>, ErrorCode>;
//     fn replace_order_internal(id: u64, new_id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool) -> Result<(), ErrorCode>;
//     fn modify_order(id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool, flag3: bool) -> Result<(), ErrorCode>;
//     fn delete_order_recursive(executing_order_id: u64, flag1: bool, flag2: bool);
//     fn activate_stop_orders_level(node: Option<R>, stop_price: u64);
//     fn add_stop_order<H: Handler<'a, B>, O: OrderOps>(orders: Orders< B>, order_books: OBMap<'a, R, T>, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
//     fn remove_order(orders: Orders< B>, id: u64);
//     fn add_market_order<H: Handler>(order_books: OBMap<'a, R, T>, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
// }

pub struct MarketExecutor;

pub fn add_order<'a, E, H, O, OC, R, T>(orders: Orders<'a, B>, order_books: OBMap<'a, R, T>, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution<'a, B, H>,
    R: Ref<'a>,
    B: MutableBook<'a>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    OC: OrderBookContainer<'a, B, H, T>,
    T: Tree<'a, R>
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
            E::add_market_order::<H>(order_books, order, matching, recursive)
        },
        OrderType::Limit => {
            E::add_limit_order::<H>(order, matching, order_books, recursive)
        },
        OrderType::Stop | OrderType::TrailingStop => {
            E::add_stop_order::<H, O>(orders, order_books, order, matching, recursive)
        },
        OrderType::StopLimit | OrderType::TrailingStopLimit => {
            E::add_stop_limit_order::<H, O>(order_books, orders, order, matching, recursive)
        },
        _ => Err(ErrorCode::OrderTypeInvalid),
    }
}


pub fn add_market_order<'a, E, H, B, T, O, R, OC>(order_book: B, order_books: OBMap<'a, R, T>, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    let mut order_book = OC::get_order_book(order_books, &order.symbol_id).expect("order book not found");

    // let some_condition = true;
    // if some_condition {
    //     matching = true;
    //     recursive = false;
    // }

    H::on_add_order(&order);

    if matching && !recursive {
        E::match_market(order_book, &order);
    }

    H::on_delete_order(&order);

    if matching && !recursive {
        E::match_order_book::<H>(order_book); // Assuming match_order also returns a Result
    }
    
    let mut order_book = B::reset_matching_price(order_book);

    Ok(())
}

pub fn execute_matching_chain<'a, E, H, T, O, B, R, OC>(order_book: B, mut level_node: Option<R>, price: u64, mut volume: u64) 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    // the overhead of ref counting and whatnot not really needed except for the tree integrity it seems
    while volume > 0 {
        if let Some(current_level) = level_node {
            let mut executing_order = (*current_level.borrow_mut()).orders.front_mut();
          //  let mut executing_order = current_level.orders.front_mut();

            while volume > 0 {
                if let Some(order) = executing_order {
                    let quantity = if order.is_aon() {
                        order.leaves_quantity
                    } else {
                        std::cmp::min(order.leaves_quantity, volume)
                    };

                    H::on_execute_order(&order, price, quantity);
                    // Switch to the next price level
                    B::update_last_price(order_book, *order, price);
                    B::update_matching_price(order_book, *order, price);
                    
                    order.executed_quantity += quantity;
                    // Reduce the executing order in the order book
                    E::reduce_order(order_book, order.id, quantity, true, false);

                    volume -= quantity;
                    executing_order = order.next_mut();
                } else {
                    break;
                }
            }
            // Assuming `get_next_level_node` returns an Level
            if let Some(next_level) = T::get_next_level_node(current_level) {
                level_node = Some(next_level);
            } else {
                break;
            } 
        } else {
            break;
        }
    }
}

pub fn match_limit<'a, E, T, H, O, B, R, OC>(order_book: B, order: Order<R>) 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    // Match the limit order
    E::match_order(order_book, order);
}

pub fn match_market<'a, E, T, H, O, B, R, OC>(mut order_book: B, mut order: Order<R>) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    // Calculate acceptable market order price with optional slippage value
    match order.order_type {
        OrderType::Buy | OrderType::Market => {
            order.price = (B::best_ask(order_book).expect("best ask not retrieved").borrow_mut()).price.saturating_add(order.slippage);
        },
        _ => {
            order.price = (B::best_bid(order_book).expect("best bid not retrieved").borrow_mut()).price.saturating_sub(order.slippage);
        },
    }

    E::match_order(order_book, order);

    Ok(())
}

pub fn match_order<'a, E, H, T, O, B, R, OC>(mut order_book: B, mut order: Order<R>) 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    let level_node: Option<R>;
    let arbitrage = if order.is_buy() {
        level_node = B::best_ask(order_book);
        order.price >= (*level_node.expect("best ask not retrieved").borrow()).price
    } else {
        level_node = B::best_bid(order_book);
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
        
        B::update_last_price(order_book, order, order.price);
        B::update_matching_price(order_book, order, order.price);
        
        order.executed_quantity += order.leaves_quantity;
        order.leaves_quantity = 0;

        return;
    }

    let mut executing_order = (*level_node.expect("best ask not retrieved").borrow()).level.orders.front();

    // Execute crossed orders
    while let Some(order) = executing_order {

        // get the execution quantity
        let quantity = order.leaves_quantity.min(order.leaves_quantity);

        // Special case for ll-Or-None' orders
        if order.is_aon() && (order.leaves_quantity > order.leaves_quantity) {
            return;
        }

        // get the execution price
        let price = order.price;

        // Call the corresponding MarketHandler
        H::on_execute_order(&order, quantity, price);

        // Update the corresponding market price
        B::update_matching_price(order_book, *order, order.price);

        // Increase the order executed quantity
        order.executed_quantity += quantity;

        // Reduce the executing order in the order book
        B::reduce_order(order_book, order, quantity, 0, 0);
        
        // Call the corresponding MarketHandler
        H::on_execute_order(&order, price, quantity);

        // Update the corresponding market price
        B::update_last_price(order_book, *order, order.price);
        B::update_matching_price(order_book, *order, order.price);

        // Increase the order executed quantity
        order.executed_quantity += quantity;

        // Reduce the order leaves quantity
        order.leaves_quantity -= quantity;
        if order.leaves_quantity == 0 {
            return;
        }
        
        let next_executing_order = order.next_mut();
        
        // Move to the next order to execute at the same price level
        if let Some(node) = next_executing_order {
            executing_order = Some(node);
        } else {
            break;
        }
    }   
}

pub fn match_order_book<'a, E, H, O, T, B, R, OC>(order_book: B)
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    loop {
        // Check if the best bid price is higher than or equal to the best ask price
        while let (Some(bid_level_node), Some(ask_level_node)) = 
            (B::best_bid(order_book), B::best_ask(order_book)) {
            // Break the loop if bid price is lower than ask price (no arbitrage opportunity)
            let (bid_level, ask_level) = (*bid_level_node.borrow(), ask_level_node.borrow());
            if bid_level.price < ask_level.price {
                break;
            }

            // Retrieve the front self.orders of both bid and ask levels
            let mut bid_order = bid_level.orders.front();
            let mut ask_order = ask_level.orders.front();

            // Process each pair of bid and ask self.orders
            while let (Some(bid_node_handle), Some(ask_node_handle)) = (bid_order, ask_order) {
                let next_bid_order = bid_node_handle.next_mut();
                let next_ask_order = ask_node_handle.next_mut();
                // Check for All-Or-None (AON) self.orders and handle them separately
                if bid_node_handle.is_aon() || ask_node_handle.is_aon() {
                    // Calculate the matching chain for AON self.orders
                    let chain = E::calculate_matching_chain_cross_levels(order_book, Some(bid_level_node), Some(ask_level_node));

                    // If no matching chain is found, exit the function
                    if chain == 0 {
                        return;
                    }

                    // Execute matching chains for AON self.orders
                    if bid_node_handle.is_aon() {
                        let price = bid_node_handle.price;
                        E::execute_matching_chain(order_book, Some(bid_level_node), price, chain);
                        E::execute_matching_chain(order_book, Some(ask_level_node), price, chain);
                    } else {
                        let price = ask_node_handle.price;
                        E::execute_matching_chain(order_book, Some(ask_level_node), price, chain);
                        E::execute_matching_chain(order_book, Some(bid_level_node), price, chain);
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
                H::on_execute_order(&executing_order, price, quantity);
                B::update_last_price(order_book, *executing_order, price);
                B::update_matching_price(order_book, *executing_order, price);
                
                // Update the executed order's quantity
                executing_order.executed_quantity += quantity;
                // Reduce the quantity of the executing order
                E::delete_order_recursive(executing_order.id, true, false);
                
                // Execute the reducing order
                H::on_execute_order(&reducing_order, price, quantity);
                
                B::update_last_price(order_book, *reducing_order, price);
                B::update_matching_price(order_book, *reducing_order, price);
                
                // Update the reducing order's quantity
                reducing_order.executed_quantity += quantity;

                // Decrease the leaves quantity of the executing order
                executing_order.leaves_quantity -= quantity;

                // Move to the next pair of self.orders at the same level
                bid_order = next_bid_order.as_deref();
                ask_order = next_ask_order.as_deref();
            }
            
            E::activate_stop_orders_level(B::best_buy_stop(order_book), B::get_market_ask_price(order_book));
            
            E::activate_stop_orders_level(B::best_sell_stop(order_book), B::get_market_bid_price(order_book));
        }

        // Keep activating stop self.orders until no more can be activated
        if !E::activate_stop_orders(order_book) {
            break;
        }
    }
}


extern crate generational_arena;

fn calculate_matching_chain_single_level<'a, T, E, H, O, B, R, OC>(mut order_book: B, mut level_node: Option<R>, price: u64, volume: u64) -> u64 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    let mut available = 0;
    // avoid panics later
   // let mut level_node = *level_node.expect("rc node failed").borrow();
    let mut order = (*level_node.expect("rc node failed").borrow()).orders.front();

    while let Some(level_handle) = level_node {

        let handle = *level_node.expect("rc node failed").borrow();
        // Check the arbitrage bid/ask prices
        let arbitrage = if handle.is_bid() {
            price <= handle.price
        } else {
            price >= handle.price
        };

        if !arbitrage {
            return 0;
        }
        
        // Travel through self.orders at current price levels

        while let Some(node) = order {

            let need = volume - available;

            let quantity = if node.is_aon() {
                node.leaves_quantity
            } else {
                std::cmp::min(node.leaves_quantity, need)
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

            // // Now you can safely call `next_mut()` on the locked node
            let next_node = node.next_mut();

            if let Some(next_node_handle) = next_node {
                order = Some(next_node_handle);
            } else {
                break;
            }
        }

        // Switch to the next price level
        if let Some(next_level_handle) = B::get_next_level_node(order_book, level_handle) {
            // let level_borrow = *level_handle.borrow_mut();
            level_node = Some(next_level_handle);
        } else {
            break;
        }
    }
    // Matching is not available
    0
}

fn calculate_matching_chain_cross_levels<'a, T, E, H, O, B, R, OC>(mut order_book: B, bid_level: A, ask_level: A) -> u64 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    let mut longest_level_handle = bid_level;
    let mut shortest_level_handle = ask_level;
    // avoid panic
    let mut longest_order = (*bid_level.borrow()).orders.front();
    let mut shortest_order = (*ask_level.borrow()).orders.front();
    let mut required = longest_order.expect("longest order not found").leaves_quantity;
    let mut available = 0;

    // Find the initial longest order chain
    if let (Some(longest), Some(shortest)) = (longest_order, shortest_order) {
        if longest.is_aon() && shortest.is_aon() {
            if shortest.leaves_quantity > longest.leaves_quantity {
                required = shortest.leaves_quantity;
                available = 0;
                std::mem::swap(&mut longest_level_handle, &mut shortest_level_handle);
                std::mem::swap(&mut longest_order, &mut shortest_order);
            }
        } else if shortest.is_aon() {
            required = shortest.leaves_quantity;
            available = 0;
            std::mem::swap(&mut longest_level_handle, &mut shortest_level_handle);
            std::mem::swap(&mut longest_order, &mut shortest_order);
        }
    }

    let mut longest_level_handle = Some(longest_level_handle);
    let mut shortest_level_handle = Some(shortest_level_handle);

    // Travel through price levels
    while let (Some(bid_level), Some(ask_level)) = (longest_level_handle, shortest_level_handle) {
        while let (Some(bid_order), Some(ask_order)) = (longest_order, shortest_order) {
            let need = required.saturating_sub(available);
            let short_order = shortest_order.expect("shortest order not found");
            let quantity = if short_order.is_aon() {
                short_order.leaves_quantity
            } else {
                std::cmp::min(short_order.leaves_quantity, need)
            };
            available += quantity;

            if required == available {
                return required;
            }

            if required < available {
                // avoid panics in future
                let next = longest_order.expect("longest order not found").next_mut();
                longest_order = shortest_order;
                shortest_order = next.as_deref();
                std::mem::swap(&mut required, &mut available);
                continue;
            }
        }

        B::get_next_level_node(order_book, bid_level);
        //  longest_order = longest_level_handle.and_then(|level| level.orders.front());
        let mut longest_order = None;

        if let Some(ref level_handle) = longest_level_handle {
            let level = (*level_handle.borrow()).level;
            if let Some(order) = level.orders.front() {
                longest_order = Some(order); // Clone the order node
            }
        }

        B::get_next_level_node(order_book, ask_level);
        if let Some(ref level_handle) = shortest_level_handle {
            let level = (*level_handle.borrow()).level; // Lock the level
            if let Some(order) = level.orders.front() {
                shortest_order = Some(order); // Clone the order node
            }
        }
    }
    0
}

pub fn add_stop_order<'a, E, T, H, O, B, R, OC>(orders: Orders<'a, B>, order_books: OBMap<'a, R, T>, mut order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    // remove panicking behavior from code
    let mut order_book = OC::get_order_book(order_books, &order.symbol_id).expect("order book not found");

    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        order.stop_price = B::calculate_trailing_stop_price(order_book, order);
    }

    H::on_add_order(&order);

    if matching && !recursive {
        let stop_price = if order.is_buy() {
            B::get_market_ask_price(order_book)
        } else {
            B::get_market_bid_price(order_book)
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

            H::on_update_order(&order);
            E::match_market(order_book, &order);
            H::on_delete_order(&order);
            if matching && !recursive {
                E::match_order_book::<H>(order_book);
            }
            
            B::reset_matching_price(order_book);

            return Ok(());
        }
    }

    if order.leaves_quantity > 0 {
        let order = Order::new(order);
        if O::insert_order(orders, &order.id, order).is_some() {
            // Order duplicate
            H::on_delete_order(&order);
            //order_pool.release(order);
        }
    
        // Add the new stop order into the order book
        if order.is_trailing_stop() || order.is_trailing_stop_limit() {
            B::add_trailing_stop_order(order_book, &order)
        } else {
            B::add_stop_order(order_book, &order)
        }
    } else {
        H::on_delete_order(&order);
    }

    if matching && !recursive {
        E::match_order_book::<H>(order_book);
    }
    
    B::reset_matching_price(order_book);

    Ok(())
}

pub fn add_stop_limit_order<'a, E, H, B, R, OC>(order_books: OBMap<'a, R, T>, orders: Orders<'a, B>, mut order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, H, R,>,
    OC: OrderBookContainer<'a, B, H>
{
    // get the valid order book for the order
    let mut order_book = OC::get_order_book(order_books, &order.id).expect("order book not found");

    // Recalculate stop price for trailing stop self.orders
    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        let diff = order.price as u64 - order.stop_price as u64;
        let mut level_update = B::calculate_trailing_stop_price(order_book, order);
        order.price = (order.stop_price as u64 + diff) as u64;
    }

    // Call the corresponding MarketHandler
    H::on_add_order(&order);

    // Automatic order matching
    if matching && !recursive {
        // Find the price to match the stop-limit order
        let stop_price = if order.is_buy() {
            B::get_market_ask_price(order_book)
        } else {
            B::get_market_bid_price(order_book)
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
            H::on_update_order(&order);

            // Match the limit order
            E::match_limit(order_book, &order);

            // Add a new limit order or delete remaining part in case of 'Immediate-Or-Cancel'/'Fill-Or-Kill' order
            if order.leaves_quantity > 0 && !order.is_ioc() && !order.is_fok() {
                // Create a new order
                let order = Order::new(order);
                if O::insert_order(orders, &order.id, order).is_some() {
                    H::on_delete_order(&order);
                    // order_pool.release(order);
                    // Handle duplicate order case here, if needed
                } else {
                    E::update_level(order_book, B::add_order(order_book, &order));
                }
            } else {
                // Call the corresponding MarketHandler
                H::on_delete_order(&order);
            }

            // Automatic order matching
            if matching && !recursive {
                E::match_order_book::<H>(order_book);
            }
            B::reset_matching_price(order_book);
        }
    }

    // Add a new stop order
    if order.leaves_quantity > 0 {
        // Insert the order
        let order = Order::new(order);
        if O::insert_order(orders, &order.id, order).is_some() {
            // Order duplicate
            H::on_delete_order(&order);
            // order_pool.release(// order.new(&Order<R>));
        }
        // Add the new stop order into the order book
        if order.is_trailing_stop() || order.is_trailing_stop_limit() {
            B::add_trailing_stop_order(order_book, &order);
        } else {
            B::add_stop_order(order_book, &order);
        }
    } else {
        // Call the corresponding MarketHandler
        H::on_delete_order(&order);
    }

    // Automatic order matching
    if matching && !recursive {
        E::match_order_book::<H>(order_book);
    }

    B::reset_matching_price(order_book);

    Ok(())
}


pub fn execute_order<'a, E, H, T, O, B, R, OC>(orders: Orders<'a, B>, order_book: B, order_books: OBMap<'a, R, T>, id: u64, price: u64, quantity: u64, matching: bool) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    let mut order = E::get_order(orders, id).map_err(|_| ErrorCode::OrderNotFound)?;

    let mut order_book = OC::get_order_book(order_books, &order.symbol_id).expect("order book not received");

    let quantity = std::cmp::min(quantity, order.leaves_quantity);
    H::on_execute_order(&order, order.price, quantity);
    B::update_last_price(order_book, order, order.price);
    B::update_matching_price(order_book, order, order.price);

    let hidden = order.hidden_quantity();
    let visible = order.visible_quantity();
    order.executed_quantity += quantity;
    order.leaves_quantity -= quantity;

    let hidden_delta = hidden - order.hidden_quantity();
    let visible_delta = visible - order.visible_quantity();

    match order.order_type {
        OrderType::Limit => {
            E::update_level(order_book, B::reduce_order(order_book, &order, quantity, hidden, visible));
        },
        OrderType::Stop | OrderType::StopLimit => { 
            B::reduce_stop_order(order_book, &order, quantity, hidden_delta, visible_delta);
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            B::reduce_trailing_stop_order(order_book, &order, quantity, hidden_delta, visible_delta);
        },
        OrderType::Buy => todo!(),
        OrderType::Market => todo!(),
    }

    if order.leaves_quantity > 0 {
        H::on_update_order(&order);
    } else {
        H::on_delete_order(&order);
        E::remove_order(orders, id);
        // order_pool.release(orders.get_mut(&id).ok_or(ErrorCode::OrderNotFound)?);
    }

    if matching {
        E::match_order_book::<H>(order_book);
    }
    B::reset_matching_price(order_book);

    Ok(())
}

pub fn mitigate_order<'a, B, E, OC, H, O, T>(id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    OC: OrderBookContainer<'a, B, H, T>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
{
    E::modify_order(id, new_price, new_quantity, true, true, false)
}

pub fn replace_order_id<'a, B, H, O, E, OC, T>(symbols: Vec<u64>,id: u64, new_id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    OC: OrderBookContainer<'a, B, H, T>,
{
    E::replace_order_internal(id, new_id, new_price, new_quantity, true, false)
}

pub fn modify_order<'a, B, E, OC, H, O, T>(mut orders: Orders<'a, B>, id: u64, new_price: u64, new_quantity: u64, mitigate: bool, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    OC: OrderBookContainer<'a, B, H, T>,
{
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if new_quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }

    // Retrieve and modify the order
    // handle with errorcode going forward
    let mut order = E::get_order(orders, id)?;
    
    if order.order_type != OrderType::Limit {
        return Err(ErrorCode::OrderTypeInvalid);
    }

    // Apply the modifications
    order.price = new_price;
    order.quantity = new_quantity;
    order.leaves_quantity = new_quantity;

    // In-Flight Mitigation (IFM) logic
    if mitigate {
        order.leaves_quantity = new_quantity.saturating_sub(order.executed_quantity);
    }

    // Handle the updated order
    if order.leaves_quantity > 0 {
        // Handle the case where the order is still active
        // e.g., H::on_update_order(&Order<R>);
    } else {
        // Handle the case where the order is now fully executed
        // e.g., H::on_delete_order(&Order<R>);
        E::remove_order(orders, id);
    }

    // Implement matching logic, if required
    if matching && !recursive {
        // Implement logic for matching orders after modification
        // This might involve interacting with an order book or a matching engine
    }

    Ok(())
}

pub fn delete_order_recursive<'a, E, T, H, O, B, R, OC>(id: u64, matching: bool, recursive: bool, order_books: OBMap<'a, R, T>, orders: Orders<'a, B>) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    // Validate parameters
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // get the order to delete
    let order = E::get_order(orders, id)?;

    // get the valid order book for the order
    // use error code possibly
    let order_book = order_books.get(&order.id).ok_or(ErrorCode::OrderBookNotFound).expect("order book");

    // Delete the order from the order book
    match order.order_type {
        OrderType::Limit => {
            E::update_level(*order_book, B::delete_order(*order_book, &order));
        },
        OrderType::Stop | OrderType::StopLimit => {
            B::delete_stop_order(*order_book, &order);
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            B::delete_trailing_stop_order(*order_book, &order);
        },
        _ => return Err(ErrorCode::OrderTypeInvalid),
    };

    // Call the corresponding MarketHandler
    H::on_delete_order(&order);

    // Erase the order
    O::remove_order(orders, &id);

    // Release the order
    // order_pool.release(order);

    // Automatic order matching
    if matching && !recursive {
        E::match_order_book::<H>(*order_book);
    }

    B::reset_matching_price(*order_book);

    // Reset matching price
    Ok(())
}

pub fn modify_order_volumes<'a, OC, E, H, O, R, T>(orders: Orders<'a, B>, id: u64, quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    OC: OrderBookContainer<'a, B, H, T>,
    T: Tree<'a, R>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
{
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }

    // Retrieve the order node
    let mut order = E::get_order(orders, id).expect("order");

    // Since MarketExecutor deals with limit orders, assume it has its way of handling them.
    // Here, we focus on the logic specific to reducing a limit order.

    let quantity_to_reduce = std::cmp::min(quantity, order.leaves_quantity);
    order.leaves_quantity -= quantity_to_reduce;

    if order.leaves_quantity > 0 {
        // Handle the case where the order is partially filled
        H::on_update_order(&order);
        // Any additional logic for updating the order goes here
    } else {
        // Handle the case where the order is fully executed
        H::on_delete_order(&order);
        O::remove_order(orders, &id); // Remove the order from the collection
        // Any additional logic for removing the order goes here
    }

    // Matching logic, if required
    if matching && !recursive {
        // Implement the logic for matching orders after reduction
        // This might involve interacting with an order book or a matching engine
    }

    Ok(())
}

pub fn reduce_order<'a, E, T, H, O, B, R, OC>(order_books: OBMap<'a, R, T>, mut order: Order<R>, id: u64, quantity: u64, matching: bool, recursive: bool, orders: Orders<'a, B>) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }
    let mut order_book = OC::get_order_book(order_books, &id).expect("order book not found");
    // let mut order = E::get_order(id);

    let quantity = min(quantity, order.leaves_quantity);
    order.leaves_quantity -= quantity;

    let hidden = order.hidden_quantity();
    let visible = order.visible_quantity();

    // Update the order or delete the empty order
    if order.leaves_quantity > 0 {
        H::on_update_order(&order);
        //let order = order.new(&order);

        // market order book into full order book
        match order.order_type {
            OrderType::Limit => {
                B::reduce_trailing_stop_order(order_book, &order, quantity, hidden, visible);
            },
            OrderType::Stop | OrderType::StopLimit => {
                B::reduce_trailing_stop_order(order_book, &order, quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                B::reduce_trailing_stop_order(order_book, &order, quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        };
    } else {
        H::on_delete_order(&order);
        match order.order_type {
            OrderType::Limit => {
                E::update_level(order_book, B::reduce_order(order_book, &order, quantity, hidden, visible));
            },
            OrderType::Stop | OrderType::StopLimit => {
                B::reduce_stop_order(order_book, &order, quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                B::reduce_trailing_stop_order(order_book, &order, quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        }

        // Erase the order
        O::remove_order(orders, &id);
        // Release the Order<R>, assuming we have an order pool with a release method
        // order_pool.release(order);
    }

    if matching && !recursive {
        E::match_order_book::<H>(order_book);
    }
    
    B::reset_matching_price(order_book);
    
    Ok(())
}

pub fn replace_order<'a, B, H, O, T, E, B, OC>(order_books: OBMap<'a, R, T>, orders: Orders<'a, B>, id: u64, order: Order<R>, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    // Delete the previous order by Id
    let order_book = order_books.get(&id).expect("order book not found");
    let order = orders.get(&id).expect("order not found");
    B::delete_order(*order_book, order);
    
    B::add_order(*order_book, order);
    Ok(())
}

pub fn replace_order_internal<'a, E, T, H, O, B, R, OC>(id: u64, new_id: u64, new_price: u64, new_quantity: u64, matching: bool, recursive: bool, orders: Orders<'a, B>, order_books: OBMap<'a, R, T>) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{
    // Validate parameters 
    if id == 0 || new_id == 0 || new_quantity == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // Retrieve the order to replace
    let order = E::get_order(orders, id).expect("order not found");
    if !order.is_trailing_stop() && !order.is_trailing_stop_limit() {
        return Err(ErrorCode::OrderTypeInvalid);
    }

    // Retrieve the valid order book for the order
    let mut order_book = OC::get_order_book(order_books, &order.id)?;

    // Delete the trailing stop order from the order book
    B::delete_trailing_stop_order(order_book, &order);

    // Replace the order
    let new_order = Order {
        id: new_id,
        price: new_price,
        quantity: new_quantity,
        executed_quantity: 0,
        leaves_quantity: new_quantity,
        ..order // Clone other fields from the existing order
    };

    // Insert the new order into the manager's collection
    if O::insert_order(orders, &new_id, new_order).is_some() {
        return Err(ErrorCode::OrderDuplicate);
    }

    // Add the new order into the order book
    B::add_trailing_stop_order(order_book, &orders[&new_id]);

    // Handle automatic order matching if required
    if matching && !recursive {
        E::match_order_book::<H>(order_book);
    }

    // Reset matching price in the order book
    B::reset_matching_price(order_book);

    Ok(())
}

