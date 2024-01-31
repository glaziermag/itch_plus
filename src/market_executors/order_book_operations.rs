use std::collections::HashMap;


use crate::market_handler::Handler;
use crate::levels::level::{LevelUpdate, UpdateType};
use crate::order_book::order_book::OrderBook;
use crate::orders::order::ErrorCode;

use std::ops::{Deref, DerefMut};

pub struct OrderBooks<'a> {
    books: HashMap<u64, OrderBook<'a>>,
}

impl<'a> Deref for OrderBooks<'a> {
    type Target = HashMap<u64, OrderBook<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.books
    }
}

impl<'a> DerefMut for OrderBooks<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.books
    }
}

impl<'a> OrderBooks<'a>  {
    pub fn add_order_book(&mut self, symbol: u64, order_book: OrderBook) -> Result<(), ErrorCode> {
        if self.contains_key(&symbol) {
            Err(ErrorCode::OrderBookDuplicate)
        } else {
            self.insert(symbol, order_book);
            Ok(())
        }
    }

    pub fn insert_order_book(&self, symbols: Vec<u64>, mut symbol: u64) -> Result<(), ErrorCode> {
        // Check if the symbol exists
        if !symbols.contains(&symbol) {
            return Err(ErrorCode::SymbolNotFound);
        }

        // Check for existing OrderBook
        if self.contains_key(&symbol) {
            return Err(ErrorCode::OrderBookDuplicate);
        }

        // Create a new OrderBook
        // Assuming OrderBook::new() does not require a weak reference to MarketExecutor
        let order_book = OrderBook::new();

        // Insert the new OrderBook into the HashMap
        self.insert(symbol, order_book);

        Ok(())
    }

    pub fn get_order_book(&self, symbol: &u64) -> Result<&OrderBook, ErrorCode> {
        self.get(&symbol).ok_or(ErrorCode::OrderBookNotFound)
    }

    pub fn remove_order_book(&mut self, symbol: &u64) -> Result<OrderBook, ErrorCode> {
        self.remove(&symbol).ok_or(ErrorCode::OrderBookNotFound)
    }

    pub fn update_level<H: Handler>(&self, order_book: C,  update: LevelUpdate, market_handler: H) -> Result<(), &'static str> {
        match update.update_type {
            UpdateType::Add => H::on_add_level(order_book, update.update_type, update.top),
            UpdateType::Update => H::on_update_level(order_book, update.update_type, update.top),
            UpdateType::Delete => H::on_delete_level(order_book, update.update_type, update.top),
            _ => {
                eprintln!("Warning: Received an unexpected update type in update_level");
            },
        };
        Ok(H::on_update_order_book(order_book, update.top))
    }
}

