
use crate::{order_book::order_book::OrderBook, orders::order::{OrderNode, Order}, levels::level::Level};

pub trait MarketHandler {

}

#[derive(Clone)]
pub struct Handler {
    updates: u64,
    symbols: u64,
    max_symbols: u64,
    order_books: u64,
    max_order_books: u64,
    max_order_book_levels: u64,
    max_order_book_orders: u64,
    orders: u64,
    max_orders: u64,
    add_orders: u64,
    update_orders: u64,
    delete_orders: u64,
    execute_orders: u64,
}

impl Default for Handler {
    fn default() -> Self {
        Handler {
            updates: 0,
            symbols: 0,
            max_symbols: 1000, // You can set this to a sensible default value
            order_books: 0,
            max_order_books: 1000, // You can set this to a sensible default value
            max_order_book_levels: 100, // You can set this to a sensible default value
            max_order_book_orders: 1000, // You can set this to a sensible default value
            orders: 0,
            max_orders: 10000, // You can set this to a sensible default value
            add_orders: 0,
            update_orders: 0,
            delete_orders: 0,
            execute_orders: 0,
        }
    }
}

impl<'a> Handler {
    pub fn new(
        max_symbols: u64,
        max_order_books: u64,
        max_order_book_levels: u64,
        max_order_book_orders: u64,
        max_orders: u64
    ) -> Self {
        Handler {
            updates: 0,
            symbols: 0,
            max_symbols,
            order_books: 0,
            max_order_books,
            max_order_book_levels,
            max_order_book_orders,
            orders: 0,
            max_orders,
            add_orders: 0,
            update_orders: 0,
            delete_orders: 0,
            execute_orders: 0,
        }
    }
    
    pub fn on_add_order(&mut self, order: Order) {

        self.updates += 1;
        self.orders += 1;
        self.max_orders = std::cmp::max(self.orders, self.max_orders);
        self.add_orders += 1;

    }
    
    pub fn on_delete_order(&self, order: Order) {
        println!("Deleted order: {:?}", order);
    }

    pub fn on_delete_order_node(&self, order_node: OrderNode) {
        println!("Deleted order: {:?}", order_node);
    }

    pub fn on_delete_duplicate_order(&self, order: Order) {
        println!("Deleted order: {:?}", order);
    }

    pub fn on_delete_unmatched_order(&self, order: Order) {
        println!("Deleted order: {:?}", order);
    }

    pub fn on_add_order_node(&self, order_node: OrderNode) {
        println!("Added order node: {:?}", order_node);
    }

    pub fn on_delete_order_book(&self, order_book: OrderBook) {
        println!("Deleted order book: {:?}", order_book);
    }

    pub fn on_update_order(&self, order: &Order) {
        println!("Order Updated: {:?}", order);
    }

    pub fn on_execute_order(&self, order: &Order, quantity: u64, price: u64) {
        println!("Executed order: {:?}, Quantity: {}, Price: {}", order, quantity, price);
    }

    pub fn on_execute_order_node(&self, order: Order, quantity: u64, price: u64) {
        println!("Executed order: {:?}, Quantity: {}, Price: {}", order, quantity, price);
    }

    pub fn on_add_order_book(&self, order_book: OrderBook) {
        println!("Added order book: {:?}", order_book);
        // Implement specific logic for MarketHandler when an order book is added
    }
    pub fn on_update_level(&self, order_book: OrderBook, level: &Level, top: bool) {
        println!("Updated level in order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for updating a level...
    }

    pub fn on_delete_level(&self, order_book: OrderBook, level: &Level, top: bool) {
        println!("Deleted level from order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for deleting a level...
    }

    pub fn on_add_level(&self, order_book: OrderBook, level: &Level, top: bool) {
        println!("Added level to order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for adding a level...
    }
    pub fn on_update_order_book(&self, order_book: OrderBook, top: bool) {
        println!("Updated order book: {:?}, Top: {}", order_book, top);
        // Additional logic for updating the order book...
    }
}