
use crate::{order_book::{OrderBook, OrderBookHandle}, Symbol, Order, order::{OrderNode, OrderHandle}, level::Level};

#[derive(Clone)]
pub struct MarketHandler {
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

impl Default for MarketHandler {
    fn default() -> Self {
        MarketHandler {
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

impl<'a> MarketHandler {
    pub fn new(
        max_symbols: u64,
        max_order_books: u64,
        max_order_book_levels: u64,
        max_order_book_orders: u64,
        max_orders: u64
    ) -> Self {
        MarketHandler {
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
    
    pub fn on_add_order(&mut self, order_handle: OrderHandle) {

        self.updates += 1;
        self.orders += 1;
        self.max_orders = std::cmp::max(self.orders, self.max_orders);
        self.add_orders += 1;

    }
    
    pub fn on_delete_order(&self, order_handle: OrderHandle) {
        println!("Deleted order_handle: {:?}", order_handle);
    }

    pub fn on_delete_order_node(&self, order_node: OrderNode) {
        println!("Deleted order_handle: {:?}", order_node);
    }

    pub fn on_delete_duplicate_order(&self, order_handle: OrderHandle) {
        println!("Deleted order_handle: {:?}", order_handle);
    }

    pub fn on_delete_unmatched_order(&self, order_handle: OrderHandle) {
        println!("Deleted order_handle: {:?}", order_handle);
    }

    pub fn on_add_order_node(&self, order_node: OrderNode) {
        println!("Added order node: {:?}", order_node);
    }

    pub fn on_delete_symbol(&self, symbol: &Symbol) {
        println!("Deleted symbol: {:?}", symbol);
    }

    pub fn on_delete_order_book(&self, order_book: OrderBook) {
        println!("Deleted order book: {:?}", order_book);
    }

    pub fn on_update_order(&self, order_handle: &Order) {
        println!("Order Updated: {:?}", order_handle);
    }

    pub fn on_execute_order(&self, order_handle: &Order, quantity: u64, price: u64) {
        println!("Executed order_handle: {:?}, Quantity: {}, Price: {}", order_handle, quantity, price);
    }

    pub fn on_execute_order_node(&self, order_handle: OrderHandle, quantity: u64, price: u64) {
        println!("Executed order_handle: {:?}, Quantity: {}, Price: {}", order_handle, quantity, price);
    }

    pub fn on_add_order_book(&self, order_book: OrderBook) {
        println!("Added order book: {:?}", order_book);
        // Implement specific logic for MarketHandler when an order book is added
    }
    pub fn on_update_level(&self, order_book: OrderBookHandle, level: &Level, top: bool) {
        println!("Updated level in order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for updating a level...
    }

    pub fn on_delete_level(&self, order_book: OrderBookHandle, level: &Level, top: bool) {
        println!("Deleted level from order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for deleting a level...
    }

    pub fn on_add_level(&self, order_book: OrderBookHandle, level: &Level, top: bool) {
        println!("Added level to order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for adding a level...
    }
    pub fn on_update_order_book(&self, order_book: OrderBookHandle, top: bool) {
        println!("Updated order book: {:?}, Top: {}", order_book, top);
        // Additional logic for updating the order book...
    }
}