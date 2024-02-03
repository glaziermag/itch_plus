
use std::collections::HashMap;


use crate::market_handler::Handler;
use crate::levels::level::{LevelUpdate, UpdateType};
use crate::order_book::order_book::OrderBook;
use crate::orders::order::ErrorCode;

use std::ops::{Deref, DerefMut};

pub trait OrderBookContainer<'a, C: DerefMut<Target = OrderBook<'a>>> {
    fn add_order_book(order_books: OBMap<'a, C>, symbol: u64, order_book: C) -> Result<(), ErrorCode>;
    fn get_order_book(order_books: OBMap<'a, C>, symbol: &u64) -> Result<C, ErrorCode>;
    fn remove_order_book(order_books: OBMap<'a, C>, symbol: &u64) -> Result<C, ErrorCode>;
    fn update_level<H: Handler>(order_books: OBMap<'a, C>, order_book: C, update: LevelUpdate) -> Result<(), &'static str>;
}

pub type OBMap<'a, C> = OrderBooks<'a, C>;
pub struct OrderBooks<'a, C: DerefMut<Target = OrderBook<'a>>> {
    books: HashMap<u64, C>,
}

impl<'a, C: DerefMut<Target = OrderBook<'a>>> Deref for OrderBooks<'a, C> {
    type Target = HashMap<u64, C>;

    fn deref(&self) -> &Self::Target {
        &self.books
    }
}

impl<'a, C: DerefMut<Target = OrderBook<'a>>> DerefMut for OrderBooks<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.books
    }
}

impl<'a, C> OrderBookContainer<'a, C> for OrderBooks<'a, C>
where
    C: DerefMut<Target = OrderBook<'a>> + Clone,
{
    fn add_order_book(mut order_books: OBMap<'a, C>, symbol: u64, order_book: C) -> Result<(), ErrorCode> {
        if order_books.books.contains_key(&symbol) {
            Err(ErrorCode::OrderBookDuplicate)
        } else {
            order_books.books.insert(symbol, order_book);
            Ok(())
        }
    }

    fn get_order_book(order_books: OBMap<'a, C>, symbol: &u64) -> Result<C, ErrorCode> {
        order_books.books.get(symbol).ok_or(ErrorCode::OrderBookNotFound).cloned()
    }

    fn remove_order_book(mut order_books: OBMap<'a, C>, symbol: &u64) -> Result<C, ErrorCode> {
        order_books.books.remove(symbol).ok_or(ErrorCode::OrderBookNotFound)
    }

    fn update_level<H: Handler>(order_books: OBMap<'a, C>, order_book: C, update: LevelUpdate) -> Result<(), &'static str> {
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

