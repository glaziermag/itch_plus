
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::levels::level::LevelUpdate;

use crate::order_book::order_book::OrderBook;
use crate::orders::order::ErrorCode;


pub trait OrderBookContainer{
    fn add_order_book(&mut self, symbol: u64, order_book: OrderBook) -> Result<(), ErrorCode>;
    fn get_order_book(&mut self, symbol: &u64) -> Result<&mut OrderBook, ErrorCode>;
    fn remove_order_book(&mut self, symbol: &u64) -> Result<OrderBook, ErrorCode>;
}

#[derive(Default)]
pub struct OBMap
{
    books: HashMap<u64, OrderBook>,
}

impl Deref for OBMap {
    type Target = HashMap<u64, OrderBook>;

    fn deref(&self) -> &Self::Target {
        &self.books
    }
}

impl DerefMut for OBMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.books
    }
}


impl OrderBookContainer for OBMap
{
    fn add_order_book(&mut self, symbol: u64, order_book: OrderBook) -> Result<(), ErrorCode> {
        if self.contains_key(&symbol) {
            Err(ErrorCode::OrderBookDuplicate)
        } else {
            self.insert(symbol, order_book);
            Ok(())
        }
    }

    fn get_order_book(&mut self, symbol: &u64) -> Result<&mut OrderBook, ErrorCode> {
        self.get_mut(symbol).ok_or(ErrorCode::OrderBookNotFound)
    }

    fn remove_order_book(&mut self, symbol: &u64) -> Result<OrderBook, ErrorCode> {
        self.remove(symbol).ok_or(ErrorCode::OrderBookNotFound)
    }
}

