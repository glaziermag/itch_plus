use std::collections::HashMap;

use crate::orders::order::OrderType;
use crate::levels::level::{LevelUpdate, UpdateType};
use crate::market_executors::executor::MarketExecutor;
use crate::order_book::order_book::OrderBook;
use crate::orders::order::ErrorCode;

pub(crate) trait OrderBooks {
    fn add_order_book(&mut self, order_type: OrderType, order_book: OrderBook) -> Result<(), ErrorCode>;
    fn insert_order_book(&self, symbols: Vec<u64>, symbol: u64) -> Result<(), ErrorCode>;
    fn get_order_book(&self, order_type: OrderType) -> Result<&OrderBook, ErrorCode>;
    fn remove_order_book(&mut self, order_type: OrderType) -> Result<OrderBook, ErrorCode>;
    fn update_level(&self, order_book: &OrderBook, update: LevelUpdate) -> Result<(), &'static str>;
}

impl OrderBooks for HashMap<u64, OrderBook<'_>> {
    fn add_order_book(&mut self, order_type: OrderType, order_book: OrderBook) -> Result<(), ErrorCode> {
        if self.order_books.contains_key(&order_type) {
            Err(ErrorCode::OrderBookAlreadyExists)
        } else {
            self.order_books.insert(order_type, order_book);
            Ok(())
        }
    }

    fn insert_order_book(&self, symbols: Vec<u64>, mut symbol: u64) -> Result<(), ErrorCode> {
        // Check if the symbol exists
        if !symbols.contains(&symbol) {
            return Err(ErrorCode::SymbolNotFound);
        }

        // Check for existing OrderBook
        if self.order_books.contains_key(&symbol) {
            return Err(ErrorCode::OrderBookDuplicate);
        }

        // Create a new OrderBook
        // Assuming OrderBook::new() does not require a weak reference to MarketExecutor
        let order_book = OrderBook::default();

        // Insert the new OrderBook into the HashMap
        self.order_books.insert(symbol, order_book);

        Ok(())
    }

    fn get_order_book(&self, order_type: OrderType) -> Result<&OrderBook, ErrorCode> {
        self.order_books.get(&order_type).ok_or(ErrorCode::OrderBookNotFound)
    }

    fn remove_order_book(&mut self, order_type: OrderType) -> Result<OrderBook, ErrorCode> {
        self.order_books.remove(&order_type).ok_or(ErrorCode::OrderBookNotFound)
    }

    fn update_level(&self, order_book: &OrderBook, update: LevelUpdate) -> Result<(), &'static str> {
        match update.update_type {
            UpdateType::Add => self.market_handler.on_add_level(order_book, &update.update, update.top),
            UpdateType::Update => self.market_handler.on_update_level(order_book, &update.update, update.top),
            UpdateType::Delete => self.market_handler.on_delete_level(order_book, &update.update, update.top),
            _ => {
                eprintln!("Warning: Received an unexpected update type in update_level");
            },
        };
        Ok(self.market_handler.on_update_order_book(order_book, update.top))
    }
}

