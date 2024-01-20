
extern crate generational_arena;

use crate::levels::level::LevelNode;
use crate::order_book::order_book::OrderBook;

use super::executor::MarketExecutor;

pub trait TradeManager {
    fn calculate_matching_chain_single_level(&self, order_book: &OrderBook, level_node: LevelNode, price: u64, volume: u64) -> u64;

    fn calculate_matching_chain_cross_levels(&self, order_book: &OrderBook, bid_level_node: LevelNode, ask_level_node: LevelNode) -> u64;
}

impl TradeManager for MarketExecutor {

    fn calculate_matching_chain_single_level(&self, mut order_book: &OrderBook, level_node: LevelNode, price: u64, volume: u64) -> u64 {

        let mut available = 0;
        let mut order_node = level_node.front();
        let mut level_node = Some(level_node);

        while let Some(level_handle) = level_node {

            // Check the arbitrage bid/ask prices
            let arbitrage = if level_handle.is_bid() {
                price <= level_handle.price
            } else {
                price >= level_handle.price
            };

            if !arbitrage {
                return 0;
            }
            
            // Travel through self.orders at current price levels

            while let Some(node) = order_node {

                let need = volume - available;
                //let order_node = order_node;

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
            if let Some(next_level_handle) = order_book.get_next_level(level_handle) {
                level_node = Some(next_level_handle);
            } else {
                break;
            }
        }
        // Matching is not available
        0
    }

    fn calculate_matching_chain_cross_levels(&self, mut order_book: &OrderBook, bid_level_node: LevelNode, ask_level_node: LevelNode) -> u64 {
        let mut longest_level_handle = bid_level_node;
        let mut shortest_level_handle = ask_level_node;
        let mut longest_order = bid_level_node.orders.front();
        let mut shortest_order = ask_level_node.orders.front();
        let mut required = longest_order.leaves_quantity;
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
        while let (Some(bid_level_node), Some(ask_level_node)) = (longest_level_handle, shortest_level_handle) {
            while let (Some(bid_order), Some(ask_order)) = (longest_order, shortest_order) {
                let need = required.saturating_sub(available);
                let quantity = if shortest_order.is_aon() {
                    shortest_order.leaves_quantity
                } else {
                    std::cmp::min(shortest_order.leaves_quantity, need)
                };
                available += quantity;

                if required == available {
                    return required;
                }

                if required < available {
                    let next = longest_order.next_mut();
                    longest_order = shortest_order;
                    shortest_order = next;
                    std::mem::swap(&mut required, &mut available);
                    continue;
                }
            }

            order_book.get_next_level(bid_level_node);
            //  longest_order = longest_level_handle.and_then(|level| level.orders.front());
            let mut longest_order = None;

            if let Some(ref level_handle) = longest_level_handle {
                let level = level_handle; // Lock the level
                if let Some(order_node) = level.front() {
                    longest_order = Some(order_node); // Clone the order node
                }
            }

            order_book.get_next_level(ask_level_node);
            if let Some(ref level_handle) = shortest_level_handle {
                let level = level_handle; // Lock the level
                if let Some(order_node) = level.front() {
                    shortest_order = Some(order_node); // Clone the order node
                }
            }
        }
        0
    }
}
