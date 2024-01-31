
extern crate generational_arena;

use std::rc::Rc;

use crate::levels::indexing::{Tree, RcNode};
use crate::levels::level::Level;
use crate::levels::level_node;
use crate::order_book::order_book::{BookOps};
use crate::references::Convertible;


fn calculate_matching_chain_single_level<B, T, C>(mut order_book: C, mut level_node: Option<RcNode>, price: u64, volume: u64) -> u64 
    where
        B: for<'a> BookOps<'a> + for<'a> Tree<'a>,
        C: Convertible<B>
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

fn calculate_matching_chain_cross_levels<B, C>(mut order_book: C, bid_level: RcNode, ask_level: RcNode) -> u64 
    where
        B: for<'a> BookOps<'a> + for<'a> Tree<'a>,
        C: Convertible<B>
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

        B::get_next_level_node(bid_level);
        //  longest_order = longest_level_handle.and_then(|level| level.orders.front());
        let mut longest_order = None;

        if let Some(ref level_handle) = longest_level_handle {
            let level = (*level_handle.borrow()).level;
            if let Some(order_node) = level.orders.front() {
                longest_order = Some(order_node); // Clone the order node
            }
        }

        B::get_next_level_node(ask_level);
        if let Some(ref level_handle) = shortest_level_handle {
            let level = (*level_handle.borrow()).level; // Lock the level
            if let Some(order_node) = level.orders.front() {
                shortest_order = Some(order_node); // Clone the order node
            }
        }
    }
    0
}
