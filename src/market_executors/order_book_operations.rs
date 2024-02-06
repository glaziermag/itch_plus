
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};


use crate::levels::indexing::Ref;
use crate::market_handler::Handler;
use crate::levels::level::{LevelUpdate, UpdateType};
use crate::order_book::order_book::{OrderBook, Mutable};
use crate::orders::order::ErrorCode;


pub trait OrderBookContainer<'a, R, B, H, T>
where
    R: Ref,
    B: MutableBook<'a, T>,
    H: Handler<'a, B>,
    T: Tree<'a, R>
{
    fn add_order_book(order_books: OBMap<'a, R, T>, symbol: u64, order_book: B) -> Result<(), ErrorCode>;
    fn get_order_book(order_books: OBMap<'a, R, T>, symbol: &u64) -> Result<D, ErrorCode>;
    fn remove_order_book(order_books: OBMap<'a, R, T>, symbol: &u64) -> Result<D, ErrorCode>;
    fn update_level(order_books: OBMap<'a, R, T>, order_book: B, update: LevelUpdate<R>) -> Result<(), &'static str>;
}

pub type OBMap<'a, R, T> = OrderBooks<'a, R, T>;
pub struct OrderBooks<'a, R, T> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    R: MutableBook<'a, R>, 
{
    books: HashMap<u64, M>,
}

impl<'a, R, T> Deref for OrderBooks<'a, R, T> 
where
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    type Target = HashMap<u64>;

    fn deref(&self) -> &Self::Target {
        &self.books
    }
}

impl<'a, R, T> DerefMut for OrderBooks<'a, R, T> 
where
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.books
    }
}

impl<'a, R, T, H> OrderBookContainer<'a, B, T, H> for OrderBooks<'a, R, T>
where
    H: Handler<'a, B>,
    B: MutableBook<'a>,
    T: Tree<'a, R> + Handler<'a, B>,
{
    fn add_order_book(mut order_books: OBMap<'a, R, T>, symbol: u64, order_book: B) -> Result<(), ErrorCode> {
        if order_books.books.contains_key(&symbol) {
            Err(ErrorCode::OrderBookDuplicate)
        } else {
            order_books.books.insert(symbol, order_book);
            Ok(())
        }
    }

    fn get_order_book(order_books: OBMap<'a, R, T>, symbol: &u64) -> Result<D, ErrorCode> {
        order_books.books.get(symbol).ok_or(ErrorCode::OrderBookNotFound).cloned()
    }

    fn remove_order_book(mut order_books: OBMap<'a, R, T>, symbol: &u64) -> Result<D, ErrorCode> {
        order_books.books.remove(symbol).ok_or(ErrorCode::OrderBookNotFound)
    }

    fn update_level(order_books: OBMap<'a, R, T>, order_book: B, update: LevelUpdate<R>) -> Result<(), &'static str> {
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

