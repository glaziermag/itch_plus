

use std::ops::Deref;

use crate::{order_book::order_book::{OrderBook, OrderBookOperations}, levels::{indexing::{Tree, RcNode}, level::Level}, orders::{order::{OrderNode, Order, ErrorCode, TimeInForce, OrderType}, orders::{OrderOps, Orders}}, market_handler::Handler};

use super::order_book_operations::OrderBooks;

pub trait Execution<'a> {
    fn activate_stop_order<C>(order_book: C, order_node: &OrderNode) -> bool
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn activate_stop_limit_order<C>(order_book: C, order_node: &mut OrderNode) -> bool
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn reduce_order<C>(order_book: C, order_node_id: u64, quantity: u64, hidden: bool, visible: bool)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn match_order<C>(order_book: C, order: Order)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn remove_order(id: u64);
    fn calculate_matching_chain_single_level<C>(order_book: C, level_node: Option<RcNode>, price: u64, leaves_quantity: u64)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn calculate_matching_chain_cross_levels<C>(order_book: C, bid_level_node: Option<RcNode>, ask_level_node: Option<RcNode>)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn execute_matching_chain<C>(order_book: C, level_node: Option<RcNode>, price: u64, chain: u64)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn delete_order_recursive(executing_order_id: u64, flag1: bool, flag2: bool);
    fn activate_stop_orders_level<C>(order_book: C, stop_price: u64)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn activate_stop_orders<C>(order_book: C)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn recalculate_trailing_stop_price<C>(order_book: C, best_ask_or_bid: Level)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn activate_individual_stop_orders<O, C>(order_book: C, stop_level_node: Option<RcNode>, market_price: u64, orders: O) -> bool
    where
        
        C: Deref<Target = OrderBook<'a>>,
        O: OrderOps;
    fn match_market<C>(order_book: C, order: Order)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn match_limit<C>(order_book: C, order: Order)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn update_level<C>(order_book: C, level_node: Option<RcNode>);
    fn match_order_book<H: Handler, C>(order_book: C)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn add_market_order<H: Handler, C>(order_books: OrderBooks, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn add_limit_order<H: Handler, C>(order: Order, matching: bool, order_books: OrderBooks,recursive: bool) -> Result<(), ErrorCode>
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn add_stop_order<H: Handler, O: OrderOps, C>(orders: O, order_books: OrderBooks, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn add_stop_limit_order<H: Handler, O: OrderOps, C>(order_books: OrderBooks, orders: O, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn modify_order(id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool, flag3: bool) -> Result<(), ErrorCode>;
    fn replace_order_internal(id: u64, new_id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool) -> Result<(), ErrorCode>;
    fn get_order_node(id: u64) -> Result<OrderNode<'a>, ErrorCode>;
    fn update_level_on_reduction<C>(order_book: C, order_node: OrderNode, quantity: u64, hidden: u64, visible: u64)
    where
        
        C: Deref<Target = OrderBook<'a>>;
    fn subtract_level_volumes(level_node: Option<RcNode>, order_node: &OrderNode);
    fn unlink_order(level_node: Option<RcNode>, order_node: OrderNode);
    fn reduce_trailing_stop_order<C, B>(order_book: C, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64)
    where
        
        C: Deref<Target = OrderBook<'a>>;
}

pub struct MarketExecutor;

pub fn add_order<'a, E, H, O>(orders: O, order_books: OrderBooks, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
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

pub fn add_market_order<'a, C, E, H, B, T, O>(order_book: C, order_books: OrderBooks, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution<'a>,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    let mut order_book = order_books.get_order_book(&order.symbol_id);

    // let some_condition = true;
    // if some_condition {
    //     matching = true;
    //     recursive = false;
    // }

    H::on_add_order(&order);

    if matching && !recursive {
        E::match_market(order_book, order);
    }

    H::on_delete_order(order);

    if matching && !recursive {
        E::match_order_book(order_book); // Assuming match_order also returns a Result
    }
    
    let mut order_book = B::reset_matching_price(order_book);

    Ok(())
}


pub fn execute_matching_chain<'a, E, H, T, C, O , B>(order_book: C, mut level_node: Option<RcNode>, price: u64, mut volume: u64) 
where
    E: Execution<'a>,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // the overhead of ref counting and whatnot not really needed except for the tree integrity it seems
    while volume > 0 {
        if let Some(current_level) = level_node {
            let mut executing_order = (*current_level.borrow_mut()).orders.front_mut();
          //  let mut executing_order = current_level.orders.front_mut();

            while volume > 0 {
                if let Some(order_node) = executing_order {
                    let quantity = if order_node.is_aon() {
                        order_node.order.leaves_quantity
                    } else {
                        std::cmp::min(order_node.order.leaves_quantity, volume)
                    };

                    H::on_execute_order(&order_node.order, price, quantity);
                    // Switch to the next price level
                    B::update_last_price(order_book, order_node.order, price);
                    B::update_matching_price(order_book, order_node.order, price);
                    
                    order_node.executed_quantity += quantity;
                    // Reduce the executing order in the order book
                    E::reduce_order(order_book, order_node.id, quantity, true, false);

                    volume -= quantity;
                    executing_order = order_node.next_mut();
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

pub fn match_limit<'a, E, C, T, H, O, B>(order_book: C, order: Order) 
where
    E: Execution<'a>,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // Match the limit order
    E::match_order(order_book, order);
}

pub fn match_market<'a, E, C, T, H, O, B>(mut order_book: C, mut order: Order) -> Result<(), ErrorCode> 
where
    E: Execution<'a>,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
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

pub fn match_order<'a, E, H, C, T, O, B>(mut order_book: C, mut order: Order) 
where
    E: Execution<'a>,
    H: Handler,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    let level_node: Option<RcNode>;
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
    while let Some(order_node) = executing_order {

        // get the execution quantity
        let quantity = order_node.order.leaves_quantity.min(order_node.order.leaves_quantity);

        // Special case for ll-Or-None' order_nodes
        if order_node.is_aon() && (order_node.order.leaves_quantity > order_node.order.leaves_quantity) {
            return;
        }

        // get the execution price
        let price = order_node.price;

        // Call the corresponding MarketHandler
        H::on_execute_order(&order_node.order, quantity, price);

        // Update the corresponding market price
        B::update_matching_price(order_book, order, order.price);

        // Increase the order executed quantity
        order.executed_quantity += quantity;

        // Reduce the executing order in the order book
        B::reduce_order(order_book, order_node, quantity, 0, 0);
        
        // Call the corresponding MarketHandler
        H::on_execute_order(&order, price, quantity);

        // Update the corresponding market price
        B::update_last_price(order_book, order, order.price);
        B::update_matching_price(order_book, order, order.price);

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

pub fn match_order_book<'a, E, H, C, O, T, B>(order_book: C)
where
    E: Execution<'a>,
    H: Handler,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
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
            let mut bid_order_node = bid_level.orders.front();
            let mut ask_order_node = ask_level.orders.front();

            // Process each pair of bid and ask self.orders
            while let (Some(bid_node_handle), Some(ask_node_handle)) = (bid_order_node, ask_order_node) {
                let next_bid_order_node = bid_node_handle.next_mut();
                let next_ask_order_node = ask_node_handle.next_mut();
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
                B::update_last_price(order_book, executing_order.order, price);
                B::update_matching_price(order_book, executing_order.order, price);
                
                // Update the executed order's quantity
                executing_order.executed_quantity += quantity;
                // Reduce the quantity of the executing order
                E::delete_order_recursive(executing_order.id, true, false);
                
                // Execute the reducing order
                H::on_execute_order(&reducing_order.order, price, quantity);
                
                B::update_last_price(order_book, reducing_order.order, price);
                B::update_matching_price(order_book, reducing_order.order, price);
                
                // Update the reducing order's quantity
                reducing_order.executed_quantity += quantity;

                // Decrease the leaves quantity of the executing order
                executing_order.leaves_quantity -= quantity;

                // Move to the next pair of self.orders at the same level
                bid_order_node = next_bid_order_node;
                ask_order_node = next_ask_order_node;
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

fn calculate_matching_chain_single_level<'a, T, C, E, H, O, B>(mut order_book: C, mut level_node: Option<RcNode>, price: u64, volume: u64) -> u64 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    let mut available = 0;
    // avoid panics later
   // let mut level_node = *level_node.expect("rc node failed").borrow();
    let mut order_node = (*level_node.expect("rc node failed").borrow()).orders.front();

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

        while let Some(node) = order_node {

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
                order_node = Some(next_node_handle);
            } else {
                break;
            }
        }

        // Switch to the next price level
        if let Some(next_level_handle) = B::get_next_level_node(level_handle) {
            // let level_borrow = *level_handle.borrow_mut();
            level_node = Some(next_level_handle);
        } else {
            break;
        }
    }
    // Matching is not available
    0
}

fn calculate_matching_chain_cross_levels<'a, C, T, E, H, O, B>(mut order_book: C, bid_level: RcNode, ask_level: RcNode) -> u64 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
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
            if let Some(order_node) = level.orders.front() {
                longest_order = Some(order_node); // Clone the order node
            }
        }

        B::get_next_level_node(order_book, ask_level);
        if let Some(ref level_handle) = shortest_level_handle {
            let level = (*level_handle.borrow()).level; // Lock the level
            if let Some(order_node) = level.orders.front() {
                shortest_order = Some(order_node); // Clone the order node
            }
        }
    }
    0
}

pub fn add_stop_order<'a, E, C, T, H, O, B>(orders: &Orders, order_books: &OrderBooks, mut order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution<'a>,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // remove panicking behavior from code
    let mut order_book = order_books.get_order_book(&order.symbol_id).expect("order book not found");

    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        order.stop_price = B::calculate_trailing_stop_price(order);
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
            E::match_market(order_book, order);
            H::on_delete_order(order);
            if matching && !recursive {
                E::match_order_book(order_book);
            }
            
            B::reset_matching_price(order_book);

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
            B::add_trailing_stop_order(order_book, &order_node)
        } else {
            B::add_stop_order(order_book,&order_node)
        }
    } else {
        H::on_delete_order(order);
    }

    if matching && !recursive {
        E::match_order_book(order_book);
    }
    
    B::reset_matching_price(order_book);

    Ok(())
}

pub fn add_stop_limit_order<'a, E, C, T, H, O, B>(order_books: &OrderBooks, orders: &Orders, mut order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> 
where
    E: Execution<'a>,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // get the valid order book for the order
    let mut order_book = order_books.get_order_book(&order.id);

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
                    E::update_level(order_book, B::add_order(order_node));
                }
            } else {
                // Call the corresponding MarketHandler
                H::on_delete_order(&order);
            }

            // Automatic order matching
            if matching && !recursive {
                E::match_order_book(order_book);
            }
            B::reset_matching_price(order_book);
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
            B::add_trailing_stop_order(order_book, order_node);
        } else {
            B::add_stop_order(order_book, order_node);
        }
    } else {
        // Call the corresponding MarketHandler
        H::on_delete_order(&order);
    }

    // Automatic order matching
    if matching && !recursive {
        E::match_order_book(order_book);
    }

    B::reset_matching_price(order_book);

    Ok(())
}


pub fn execute_order<'a, E, H, C, T, O, B>(order_book: C, order_books: OrderBooks, id: u64, price: u64, quantity: u64, matching: bool) -> Result<(), ErrorCode> 
where
    E: Execution<'a>,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    let mut order_node = E::get_order_node(id).ok_or(ErrorCode::OrderNotFound)?;

    let mut order_book = order_books.get_order_book(&order_node.order.symbol_id).expect("order book not received");

    let quantity = std::cmp::min(quantity, order_node.order.leaves_quantity);
    H::on_execute_order(order_node, order_node.price, quantity);
    B::update_last_price(order_node.order, order_node.price);
    B::update_matching_price(order_node.order, order_node.price);

    let hidden = order_node.order.hidden_quantity();
    let visible = order_node.order.visible_quantity();
    order_node.executed_quantity += quantity;
    order_node.order.leaves_quantity -= quantity;

    let hidden_delta = hidden - order_node.order.hidden_quantity();
    let visible_delta = visible - order_node.order.visible_quantity();

    match order_node.order_type {
        OrderType::Limit => {
            E::update_level(order_book, E::reduce_order(order_book, order_node.id, quantity, hidden, visible));
        },
        OrderType::Stop | OrderType::StopLimit => { 
            B::reduce_stop_order(order_node, quantity, hidden_delta, visible_delta);
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            E::reduce_trailing_stop_order(order_book, order_node, quantity, hidden_delta, visible_delta);
        },
        OrderType::Buy => todo!(),
        OrderType::Market => todo!(),
    }

    if order_node.order.leaves_quantity > 0 {
        H::on_update_order(&order_node.order);
    } else {
        H::on_delete_order_node(order_node);
        E::remove_order(id);
        // order_pool.release(orders.get_mut(&id).ok_or(ErrorCode::OrderNotFound)?);
    }

    if matching {
        E::match_order_book(order_book);
    }
    B::reset_matching_price(order_book);

    Ok(())
}

pub fn mitigate_order<'a, E: Execution<'a>>(id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> {
    E::modify_order(id, new_price, new_quantity, true, true, false)
}

pub fn replace_order_id<'a, E: Execution<'a>>(symbols: Vec<u64>,id: u64, new_id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> {
    E::replace_order_internal(id, new_id, new_price, new_quantity, true, false)
}

pub fn modify_order<'a, E: Execution<'a>>(mut orders: Orders, id: u64, new_price: u64, new_quantity: u64, mitigate: bool, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if new_quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }

    // Retrieve and modify the order
    // handle with errorcode going forward
    let mut order_node = E::get_order_node(id)?;
    
    if order_node.order_type != OrderType::Limit {
        return Err(ErrorCode::OrderTypeInvalid);
    }

    // Apply the modifications
    order_node.price = new_price;
    order_node.quantity = new_quantity;
    order_node.order.leaves_quantity = new_quantity;

    // In-Flight Mitigation (IFM) logic
    if mitigate {
        order_node.order.leaves_quantity = new_quantity.saturating_sub(order_node.executed_quantity);
    }

    // Handle the updated order
    if order_node.order.leaves_quantity > 0 {
        // Handle the case where the order is still active
        // e.g., H::on_update_order(&order_node.order);
    } else {
        // Handle the case where the order is now fully executed
        // e.g., H::on_delete_order(&order_node.order);
        orders.remove_order(id);
    }

    // Implement matching logic, if required
    if matching && !recursive {
        // Implement logic for matching orders after modification
        // This might involve interacting with an order book or a matching engine
    }

    Ok(())
}

pub fn delete_order_recursive<'a, E: Execution<'a>, H: Handler, C: Deref<Target = OrderBook<'a>>>(id: u64, matching: bool, recursive: bool, order_books: OrderBooks, orders: Orders) -> Result<(), ErrorCode> {
    // Validate parameters
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // get the order to delete
    let order_node = E::get_order_node(id)?;

    // get the valid order book for the order
    // use error code possibly
    let order_book = order_books.get(&order_node.id).ok_or(ErrorCode::OrderBookNotFound);

    // Delete the order from the order book
    match order_node.order_type {
        
        OrderType::Limit => {
            E::update_level(order_book, E::delete_order(order_node.id));
        },
        OrderType::Stop | OrderType::StopLimit => {
            B::delete_stop_order(order_node.id);
        },
        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
            B::delete_trailing_stop_order(order_node.id);
        },
        _ => return Err(ErrorCode::OrderTypeInvalid),
    };

    // Call the corresponding MarketHandler
    H::on_delete_order_node(&order_node);

    // Erase the order
    orders.remove_order(&id);

    // Release the order
    // order_pool.release(order_node);

    // Automatic order matching
    if matching && !recursive {
        E::match_order_book(order_book);
    }

    B::reset_matching_price(order_book);

    // Reset matching price
    Ok(())
}

pub fn modify_order_volumes<'a, E: Execution<'a>, H: Handler, O: OrderOps>(orders: O, id: u64, quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }

    // Retrieve the order node
    let order_node = E::get_order_node(&id);

    // Since MarketExecutor deals with limit orders, assume it has its way of handling them.
    // Here, we focus on the logic specific to reducing a limit order.

    let quantity_to_reduce = std::cmp::min(quantity, order_node.order.leaves_quantity);
    order_node.order.leaves_quantity -= quantity_to_reduce;

    if order_node.order.leaves_quantity > 0 {
        // Handle the case where the order is partially filled
        H::on_update_order(&order_node.order);
        // Any additional logic for updating the order goes here
    } else {
        // Handle the case where the order is fully executed
        H::on_delete_order(&order_node.order);
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

pub fn reduce_order<'a, E: Execution<'a>, H: Handler, O: OrderOps, C: Deref<Target = OrderBook<'a>>>(order_books: OrderBooks, order_node: OrderNode, id: u64, quantity: u64, matching: bool, recursive: bool, orders: O) -> Result<(), ErrorCode> {
    if id == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }
    if quantity == 0 {
        return Err(ErrorCode::OrderQuantityInvalid);
    }
    let mut order_book = order_books.get_order_book(&id);
    // let mut order_node = E::get_order_node(id);

    let quantity = min(quantity, order_node.order.leaves_quantity);
    order_node.order.leaves_quantity -= quantity;

    let hidden = order_node.order.hidden_quantity();
    let visible = order_node.order.visible_quantity();

    // Update the order or delete the empty order
    if order_node.order.leaves_quantity > 0 {
        H::on_update_order(&order_node.order);
        //let order_node = order_node.new(&order_node.order);

        // market order book into full order book
        match order_node.order_type {
            OrderType::Limit => {
                E::reduce_trailing_stop_order(order_book, &order_node, quantity, hidden, visible);
            },
            OrderType::Stop | OrderType::StopLimit => {
                E::reduce_trailing_stop_order(order_book, &order_node, quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                E::reduce_trailing_stop_order(order_book, &order_node, quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        };
    } else {
        H::on_delete_order(&order_node.order);
        match order_node.order_type {
            OrderType::Limit => {
                E::update_level(order_book, B::reduce_order(order_node, quantity, hidden, visible));
            },
            OrderType::Stop | OrderType::StopLimit => {
                B::reduce_stop_order(order_node, quantity, hidden, visible);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                E::reduce_trailing_stop_order(order_book, &order_node, quantity, hidden, visible);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        }

        // Erase the order
        orders.remove_order(&id);
        // Release the order, assuming we have an order pool with a release method
        // order_pool.release(order_node);
    }

    if matching && !recursive {
        E::match_order_book(order_book);
    }
    
    B::reset_matching_price(order_book);
    
    Ok(())
}

pub fn replace_order<'a, E: Execution<'a>, H: Handler, O: OrderOps>(order_books: OrderBooks, orders: O, id: u64, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
    // Delete the previous order by Id
    let result = E::delete_order(id.try_into(), true, false);
    if let Err(error) = result {
        return Err(error);
    }
    E::add_order(orders, order_books, order, matching, recursive)
}

pub fn replace_order_internal<'a, E: Execution<'a>, O: OrderOps, C: Deref<Target = OrderBook<'a>>>(id: u64, new_id: u64, new_price: u64, new_quantity: u64, matching: bool, recursive: bool, orders: O, order_books: OrderBooks) -> Result<(), ErrorCode> {
    // Validate parameters 
    if id == 0 || new_id == 0 || new_quantity == 0 {
        return Err(ErrorCode::OrderIdInvalid);
    }

    // Retrieve the order to replace
    let order_node = E::get_order(&id).ok_or(ErrorCode::OrderNotFound)?;
    if !order_node.is_trailing_stop() && !order_node.is_trailing_stop_limit() {
        return Err(ErrorCode::OrderTypeInvalid);
    }

    // Retrieve the valid order book for the order
    let mut order_book = order_books.get_order_book(order_node.id)?;

    // Delete the trailing stop order from the order book
    B::delete_trailing_stop_order(order_node)?;

    // Replace the order
    let new_order = Order {
        id: new_id,
        price: new_price,
        quantity: new_quantity,
        executed_quantity: 0,
        leaves_quantity: new_quantity,
        ..*order_node // Clone other fields from the existing order
    };

    // Insert the new order into the manager's collection
    if orders.insert(new_id, new_order).is_some() {
        return Err(ErrorCode::OrderDuplicate);
    }

    // Add the new order into the order book
    B::add_trailing_stop_order(&orders[&new_id])?;

    // Handle automatic order matching if required
    if matching && !recursive {
        E::match_order_book(&mut order_book)?;
    }

    // Reset matching price in the order book
    B::reset_matching_price(order_book);

    Ok(())
}

