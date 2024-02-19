use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{levels::{indexing::{LevelNode}, level::LevelUpdate}, order_book::order_book::OrderBook, orders::{order::{ErrorCode, Order}, orders::Orders}};

use super::order_book_operations::OBMap;



pub trait Execution
{
    fn activate_stop_order(order_book: &mut OrderBook, orders: &Orders, order: &Order)  -> Result<(), ErrorCode>;
    fn activate_stop_limit_order(order_book: &mut OrderBook, orders: &Orders, order: &mut Order) -> bool;
    fn reduce_order(orders: &Orders, order_id: u64, quantity: u64, hidden: bool, visible: bool) -> Option<Rc<RefCell<LevelNode>>>;
    fn match_order(order: &Order) -> Result<(), ErrorCode>;
    fn calculate_matching_chain_single_level(level_node: Option<Rc<RefCell<LevelNode>>>, price: u64, leaves_quantity: u64)  -> Result<u64, ErrorCode>;
    fn calculate_matching_chain_cross_levels(bid_level_node: Option<Rc<RefCell<LevelNode>>>, ask_level_node: Option<Rc<RefCell<LevelNode>>>)  -> Result<u64, ErrorCode>;
    fn execute_matching_chain(level_node: Option<Rc<RefCell<LevelNode>>>, price: u64, chain: u64) -> Result<(), ErrorCode>;
    fn activate_stop_orders() -> bool;
    fn recalculate_trailing_stop_price(best_ask_or_bid: Option<Rc<RefCell<LevelNode>>>) -> Result<(), ErrorCode>;
    fn activate_individual_stop_orders(stop_level_node: Option<Rc<RefCell<LevelNode>>>, market_price: u64, orders: &Orders) -> bool;
    fn match_market(order: &mut Order)-> Result<(), ErrorCode>;
    fn match_limit(order: &Order) -> Result<(), ErrorCode>;
    fn match_order_book();
    fn update_level(order_book: &OrderBook, update: LevelUpdate)-> Result<(), ErrorCode>;
    fn add_limit_order(order: &Order, matching: bool, order_books: OBMap,recursive: bool) -> Result<(), ErrorCode>;
    fn add_stop_limit_order(order_books: OBMap, orders: &Orders, order: &Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn update_level_on_reduction(order: &Order, quantity: u64, hidden: u64, visible: u64) -> Result<(), ErrorCode>;
    fn reduce_trailing_stop_order(order: &Order, quantity: u64, hidden: u64, visible: u64) -> Result<(), ErrorCode>;
    fn replace_order_internal(id: u64, new_id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool) -> Result<(), ErrorCode>;
    fn modify_order(id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool, flag3: bool) -> Result<(), ErrorCode>;
    fn delete_order_recursive(executing_order_id: u64, flag1: bool, flag2: bool);
    fn activate_stop_orders_level(node: Option<Rc<RefCell<LevelNode>>>, stop_price: u64)  -> Result<(), ErrorCode>;
    fn add_stop_order(orders: &Orders, order_books: OBMap, order: &Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn remove_order(orders: &Orders, id: u64) -> Result<(), ErrorCode>;
    fn add_market_order(order_books: OBMap, order: &Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
}

struct ExecutionContext<E> {
    // Context fields...
    _marker: PhantomData<E>,
}

impl<E> ExecutionContext<E> 
where
    E: Execution,
{
    pub fn calculate_matching_chain_cross_levels(bid_level_node: Option<Rc<RefCell<LevelNode>>>, ask_level_node: Option<Rc<RefCell<LevelNode>>>) -> Result<u64, ErrorCode> {
        E::calculate_matching_chain_cross_levels(bid_level_node, ask_level_node)
    }

    pub fn execute_matching_chain(level_node: Option<Rc<RefCell<LevelNode>>>, price: u64, chain: u64) -> Result<(), ErrorCode> {
        E::execute_matching_chain(level_node, price, chain)
    }

    pub fn activate_stop_orders() -> bool {
        E::activate_stop_orders()
    }

    pub fn recalculate_trailing_stop_price(best_ask_or_bid: Option<Rc<RefCell<LevelNode>>>) -> Result<(), ErrorCode> {
        E::recalculate_trailing_stop_price(best_ask_or_bid)
    }

    pub fn activate_individual_stop_orders(stop_level_node: Option<Rc<RefCell<LevelNode>>>, market_price: u64, orders: &Orders) -> bool{
        E::activate_individual_stop_orders(stop_level_node, market_price, orders)
    }

    pub fn match_market(order: &mut Order) -> Result<(), ErrorCode> {
        E::match_market(order)
    }

    pub fn match_limit(order: &Order) -> Result<(), ErrorCode> {
        E::match_limit(order)
    }

    pub fn match_order_book() {
        E::match_order_book();
    }

    pub fn update_level(order_book: &OrderBook, update: LevelUpdate) -> Result<(), ErrorCode> {
        E::update_level(order_book, update)
    }

    pub fn add_limit_order(order: &Order, matching: bool, order_books: OBMap, recursive: bool) -> Result<(), ErrorCode> {
        E::add_limit_order(order, matching, order_books, recursive)
    }

    pub fn add_stop_limit_order(order_books: OBMap, orders: &Orders, order: &Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        E::add_stop_limit_order(order_books, orders, order, matching, recursive)
    }

    pub fn update_level_on_reduction(order: &Order, quantity: u64, hidden: u64, visible: u64) -> Result<(), ErrorCode> {
        E::update_level_on_reduction(order, quantity, hidden, visible)
    }

    pub fn reduce_trailing_stop_order(order: &Order, quantity: u64, hidden: u64, visible: u64) -> Result<(), ErrorCode> {
        E::reduce_trailing_stop_order(order, quantity, hidden, visible)
    }

    pub fn replace_order_internal(id: u64, new_id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool) -> Result<(), ErrorCode> {
        E::replace_order_internal(id, new_id, new_price, new_quantity, flag1, flag2)
    }

    pub fn modify_order(id: u64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool, flag3: bool) -> Result<(), ErrorCode> {
        E::modify_order(id, new_price, new_quantity, flag1, flag2, flag3)
    }

    // Note: This method requires &mut self in the trait, which is inconsistent with the static method pattern.
    // For demonstration, assuming a revised version without &mut self requirement:
    pub fn delete_order_recursive(executing_order_id: u64, flag1: bool, flag2: bool) {
        E::delete_order_recursive(executing_order_id, flag1, flag2);
    }

    pub fn activate_stop_orders_level(node: Option<Rc<RefCell<LevelNode>>>, stop_price: u64) -> Result<(), ErrorCode> {
        E::activate_stop_orders_level(node, stop_price)
    }

    pub fn add_stop_order(orders: &Orders, order_books: OBMap, order: &Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        E::add_stop_order(orders, order_books, order, matching, recursive)
    }

    pub fn remove_order(orders: &Orders, id: u64) -> Result<(), ErrorCode> {
        E::remove_order(orders, id)
    }

    pub fn add_market_order(order_books: OBMap, order: &Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        E::add_market_order(order_books, order, matching, recursive)
    }
    // Additional methods that wrap E's associated functions...
}
