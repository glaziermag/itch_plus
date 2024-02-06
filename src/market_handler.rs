
use std::borrow::Borrow;

use crate::{orders::order::{Order}, levels::{level::{Level, UpdateType}}};

pub trait Handler<'a, B, R> 
where
   // B: MutableBook<'a> + Mut<'a, T>,
    R: Ref<'a>,
    B: MutableBook,
{
    // Assuming 'Order', 'OrderBook', 'Update', and 'Order' are types defined elsewhere
    fn new(
        max_symbols: u64,
        max_order_books: u64,
        max_order_book_levels: u64,
        max_order_book_orders: u64,
        max_orders: u64
    ) -> Self;
    fn on_execute_order(order: &Order<R>, price: u64, leaves_quantity: u64);
    fn on_add_level(order_book: B,  update: UpdateType, top: bool);
    fn on_update_level(order_book: B, update: UpdateType, top: bool);
    fn on_delete_level(order_book: B, update: UpdateType, top: bool);
    fn on_update_order_book(order_book: B,  top: bool);
    fn on_delete_order(order: &Order<R>);
    fn on_update_order(order: &Order<R>);
    fn on_delete_unmatched_order(order: &Order<R>);
    fn on_add_order(order: &Order<R>);
    fn on_delete_order_book(&self, order_book: B);
    fn on_add_order_book(&self, order_book: B);
}

//impl Handler for MarketHandler {}

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

impl<'a, B> Handler<'_,  B> for MarketHandler {
    fn new(
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
    
    fn on_add_order(&mut self, order: Order<R>) {

        self.updates += 1;
        self.orders += 1;
        self.max_orders = std::cmp::max(self.orders, self.max_orders);
        self.add_orders += 1;

    }
    
    fn on_delete_order(&self, order: &Order<R>) {
        println!("Deleted order: {:?}", order);
    }

    fn on_delete_unmatched_order(&self, order: &Order<R>) {
        println!("Deleted order: {:?}", order);
    }

    fn on_delete_order_book(&self, order_book: B) {
       // println!("Deleted order book: {:?}", order_book);
    }

    fn on_update_order(&self, order: &Order<R>) {
        println!("Order Updated: {:?}", order);
    }

    fn on_execute_order(&self, order: &Order<R>, quantity: u64, price: u64) {
        println!("Executed order: {:?}, Quantity: {}, Price: {}", order, quantity, price);
    }

    fn on_add_order_book(&self, order_book: B) {
      //  println!("Added order book: {:?}", order_book);
        // Implement specific logic for MarketHandler when an order book is added
    }
    fn on_update_level<C>(&self, order_book: B, level: &Level<R>, top: bool) {
     //   println!("Updated level in order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for updating a level...
    }

    fn on_delete_level<C>(&self, order_book: B, level: &Level<R>, top: bool) 
    where 
        B: MutableBook<'a>,
    {
       // println!("Deleted level from order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for deleting a level...
    }

    fn on_add_level<C>(&self, order_book: B, level: &Level<R>, top: bool) 
    where 
        B: MutableBook<'a>,
    {
      //  println!("Added level to order book: {:?}, Level: {:?}, Top: {}", order_book, level, top);
        // Additional logic for adding a level...
    }
    fn on_update_order_book<C>(&self, order_book: B, top: bool) 
    where 
        B: MutableBook<'a>,
    {
      //  println!("Updated order book: {:?}, Top: {}", order_book, top);
        // Additional logic for updating the order book...
    }
}