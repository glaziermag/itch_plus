
use core::fmt;
use std::{ops::{ Deref, DerefMut}, rc::Rc, cell::RefCell, ptr, cmp::{max, min}, borrow::{Borrow, BorrowMut}, marker::PhantomData};
use crate::{levels::{level::{UpdateType, LevelUpdate, Level, LevelType, PopCurrent, LevelOps}, indexing::{Tree, MutableBook}}, orders::{order::{Order, ErrorCode, TimeInForce, OrderType}, orders::{OrderOps, Orders}}, market_handler::Handler, market_executors::{executor::Execution, order_book_operations::{OrderBookContainer, OBMap}}};

#[derive(Debug)]
pub enum OrderBookError {
    OrderNotFound,
    LevelNotFound,
}

pub trait Mutable<T>: Deref + Borrow<T> + fmt::Pointer + DerefMut + BorrowMut<T> {}

//impl<T> Mutable<T> for &mut T {}

pub trait Ref<T>: Copy + Clone + Deref + Borrow<T> {}

//impl<T> Ref<T> for &T {}

// Trait defining operations on an OrderBook
pub trait OrderBookOperations<'a, B, E, H, R>
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    // Assuming T and U are part of the Execution trait, for example,
    // you would declare them inside the Execution trait or wherever they are actually used.
{
    fn activate_stop_orders_level(order_book: B, level: Level<'a, R>, stop_price: u64) -> bool;
    fn delete_stop_level(order_book: B, order: &Order<R>);
    fn add_stop_level(order_book: B, order: &Order<R>) -> Option<R>;
    fn best_sell_stop(order_book: B) -> Option<R>;
    fn best_buy_stop(order_book: B) -> Option<R>;
    fn reset_matching_price(order_book: B);
    fn get_market_ask_price(order_book: B) -> u64;
    fn get_market_bid_price(order_book: B) -> u64;
    fn update_last_price(order_book: B, order: Order<R>, price: u64);
    fn update_matching_price(order_book: B, order: Order<R>, price: u64);
    fn calculate_trailing_stop_price(order_book: B, order: Order<R>) -> u64;
    fn recalculate_trailing_stop_price(order_book: B, level: Level<R>);
    fn add_limit_order(orders: Orders< B>, order: Order<R>, matching: bool, order_books: OBMap<'a, R, T>, recursive: bool) -> Result<(), ErrorCode>;
    fn add_order(order_book: B, order: &Order<R>) -> LevelUpdate<'a, R>;
    fn add_stop_order(order_book: B, order: &Order<R>);
    fn add_trailing_stop_order(order_book: B, order: &Order<R>);
    fn reduce_order(order_book: B, order: &'a Order<'a, R>, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate<'a, R>; 
    fn delete_order(order_book: B, order: &'a Order<'a, R>) -> LevelUpdate<'a, R>;
    fn reduce_trailing_stop_order(order_book: B, order: &Order<R>, quantity: u64, hidden: u64, visible: u64);
    fn delete_trailing_stop_order(order_book: B, order: &Order<R>) -> Result<(), &'static str>;
    fn get_market_trailing_stop_price_ask(order_book: B) -> u64;
    fn get_market_trailing_stop_price_bid(order_book: B) -> u64;
    fn reduce_stop_order(order_book: B, order: &Order<'a, R>, quantity: u64, hidden_delta: u64, visible_delta: u64);
    fn get_next_level_node(order_book: B, level_node: A) -> Option<R>;
    fn best_trailing_buy_stop(order_book: B) -> Option<R>;
    fn best_trailing_sell_stop(order_book: B) -> Option<R>;
    fn delete_trailing_stop_level(order_book: B, order: &Order<'a, R>);
    fn activate_individual_stop_orders(order_book: B, level_node: Option<R>, market_price: u64, orders: Orders< B>) -> bool;
    fn create_and_insert_level(order_book: B, price: u64, level_type: LevelType, tree: T) -> T;
    fn best_ask(order_book: B) -> Option<R>;
    fn best_bid(order_book: B) -> Option<R>;
    fn delete_level(order_book: B, order: &Order<R>);
    fn is_top_of_book(order_book: B, order: &Order<R>) -> bool;
    fn delete_stop_order(order_book: B, order: &Order<R>); 
    fn add_volumes(order_book: B, level: Level<'a, R>, order: &Order<R>);
    fn get_next_trailing_stop_level(order_book: B, level_node: A) -> Option<R>; 
    fn add_level(order_book: B, order: &Order<R>) -> Option<R>;
    fn add_trailing_stop_level(order_book: B, order: &Order<R>) -> Option<R>;
}

impl<'a, E, O, H, T, M, A, B, R> OrderBookOperations<'a, E, O, H, T, M, A> for OrderBook<'a, R> 
where
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    R: Ref<'a>,
    // Assuming T and U are part of the Execution trait, for example,
    // you would declare them inside the Execution trait or wherever they are actually used.
{

    fn add_trailing_stop_level(mut order_book: B, order: &Order<R>) -> Option<R> {
        todo!()
    }
    fn get_next_trailing_stop_level(order_book: B, level_node: A) -> Option<R> {
        todo!()
    }
    fn add_volumes(order_book: B, level: Level<'a, R>, order: &Order<R>) {
        todo!()
    }
    fn delete_stop_order(order_book: B, order: &Order<R>) {
        todo!()
    }
    
    fn get_market_trailing_stop_price_ask(order_book: B) -> u64 {
        todo!()
    }

    fn get_market_trailing_stop_price_bid(order_book: B) -> u64 {
        todo!()
    }

    fn activate_stop_orders_level(order_book: B, level: Level<'a, R>, stop_price: u64) -> bool {
        todo!()
    }

    fn delete_stop_level(order_book: B, order: &Order<R>) {
        todo!()
    }

    fn add_stop_level(order_book: B, order: &Order<R>) -> Option<R> {
        todo!()
    }

    fn best_sell_stop(order_book: B) -> Option<R> {
        todo!()
    }

    fn best_buy_stop(order_book: B) -> Option<R> {
        todo!()
    }

    fn reset_matching_price(order_book: B) {
        todo!()
    }

    fn get_market_ask_price(order_book: B) -> u64 {
        todo!()
    }

    fn get_market_bid_price(order_book: B) -> u64 {
        todo!()
    }

    fn update_last_price(order_book: B, order: Order<R>, price: u64) {
        todo!()
    }

    fn update_matching_price(order_book: B, order: Order<R>, price: u64) {
        todo!()
    }

    fn calculate_trailing_stop_price(order_book: B, order: Order<'a, R>) -> u64 {
        todo!()
    }

    fn recalculate_trailing_stop_price(order_book: B, level: Level<R>) {
        todo!()
    }

    fn add_limit_order(orders: Orders< B>, order: Order<R>, matching: bool, order_books: OBMap<'a, R, T>, recursive: bool) -> Result<(), ErrorCode> {
        todo!()
    }

    fn add_order(order_book: B, order: &Order<R>) -> LevelUpdate<'a, R> {
        todo!()
    }

    fn add_stop_order(order_book: B, order: &Order<R>) {
        todo!()
    }

    fn add_trailing_stop_order(order_book: B, order: &Order<R>) {
        todo!()
    }

    fn reduce_order(order_book: B, order: &'a Order<'a, R>, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate<'a, R> {
        todo!()
    }

    fn delete_order(order_book: B, order: &'a Order<'a, R>) -> LevelUpdate<'a, R> {
        todo!()
    }

    fn reduce_trailing_stop_order(order_book: B, order: &Order<R>, quantity: u64, hidden: u64, visible: u64) {
        todo!()
    }

    fn delete_trailing_stop_order(order_book: B, order: &Order<R>) -> Result<(), &'static str> {
        todo!()
    }

    fn reduce_stop_order(order_book: B, order: &Order<'a, R>, quantity: u64, hidden_delta: u64, visible_delta: u64) {
        todo!(); // Replace with actual logic
    }

    fn delete_trailing_stop_level(order_book: B, order: &Order<'a, R>) {
        todo!(); // Replace with actual logic
    }

    fn get_next_level_node(order_book: B, level_node: A) -> Option<R> {
        todo!()
    }

    fn best_trailing_buy_stop(order_book: B) -> Option<R> {
        todo!()
    }

    fn best_trailing_sell_stop(order_book: B) -> Option<R> {
        todo!()
    }

    fn activate_individual_stop_orders(order_book: B, level_node: Option<R>, market_price: u64, orders: Orders< B>) -> bool {
        todo!()
    }
    fn create_and_insert_level(order_book: B, price: u64, level_type: LevelType, tree: T) -> T {
        // Placeholder: Implement the logic to create and insert a level
        todo!()
    }

    fn best_ask(order_book: B) -> Option<R> {
        // Placeholder: Return the best ask level
        todo!()
    }

    fn best_bid(order_book: B) -> Option<R> {
        // Placeholder: Return the best bid level
        todo!()
    }

    fn delete_level(order_book: B, order: &Order<R>) {
        // Placeholder: Delete a level based on the order node
        todo!()
    }

    fn is_top_of_book(order_book: B, order: &Order<R>) -> bool {
        // Placeholder: Determine if an order node is at the top of the book
        todo!()
    }

    fn add_level(order_book: B, order: &Order<R>) -> Option<R> {
        todo!()
    }
}

#[derive(Default)]
pub struct OrderBook<'a, R> 
where
    R: Ref<'a>
{
    pub best_bid: Option<R>,
    pub best_ask: Option<R>,
    pub bids: Option<R>,
    pub asks: Option<R>,

    pub best_buy_stop: Option<R>,
    pub best_sell_stop: Option<R>,
    pub buy_stop: Option<R>,
    pub sell_stop: Option<R>,

    pub(crate) last_bid_price: u64,
    pub(crate) last_ask_price: u64,
    pub(crate) matching_bid_price: u64,
    pub(crate) matching_ask_price: u64,

    pub best_trailing_buy_stop: Option<R>,
    pub best_trailing_sell_stop: Option<R>,
    pub trailing_buy_stop: Option<R>,
    pub trailing_sell_stop: Option<R>,
    pub trailing_bid_price: u64,
    pub trailing_ask_price: u64,
    pub(crate) _marker: PhantomData<&'a M>
}

impl<'a, R: Ref<'a> + 'a> OrderBook<'_, R> {

    pub fn new() -> OrderBook<'a, R> {
        OrderBook {
            best_bid: todo!(),
            best_ask: todo!(),
            bids: todo!(),
            asks: todo!(),
            best_buy_stop: todo!(),
            best_sell_stop: todo!(),
            buy_stop: todo!(),
            sell_stop: todo!(),
            last_bid_price: todo!(),
            last_ask_price: todo!(),
            matching_bid_price: todo!(),
            matching_ask_price: todo!(),
            best_trailing_buy_stop: todo!(),
            best_trailing_sell_stop: todo!(),
            trailing_buy_stop: todo!(),
            trailing_sell_stop: todo!(),
            trailing_bid_price: todo!(),
            trailing_ask_price: todo!(),
            _marker: PhantomData
        }
    }
}

//#[cfg(feature = "experimental_level_changes")]
pub fn reduce_trailing_stop_order<'a, L, R>(order: &Order<R>, quantity: u64, hidden: u64, visible: u64) 
where
    L: LevelOps<'a, R>,
    R: Ref<'a>,
{
    // Assuming we have a way to get a mutable reference to an order and its level.
    // Update the price level volume
    // remove panicking behavior from code
    match order.level_node {
        Some(node) => {
            let mut borrowed_level = node.borrow_mut().level;
            // looking to get &mut level isolation here
            L::subtract_volumes(&mut borrowed_level, order);
            if order.leaves_quantity == 0 {
                L::unlink_order(&mut borrowed_level, order)
            }
            if borrowed_level.total_volume == 0 {
                order.level_node = None
            }
        },
        None => {
            eprintln!("order level node not obtained")
        }
    }
}

// Method to get the best trailing buy stop level
pub fn best_trailing_buy_stop<'a, B>(order_book: B) -> Option<R> 
where
    B: MutableBook<'a>,
{
    (*order_book).best_trailing_buy_stop
}

// Method to get the best trailing sell stop level
pub fn best_trailing_sell_stop<'a, B>(order_book: B) -> Option<R> 
where
    B: MutableBook<'a>,
{
    (*order_book).best_trailing_sell_stop
}

pub fn get_trailing_buy_stop_level<'a, B, R>(order_book: B, price: u64) -> Option<R> 
where
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    //((*order_book).trailing_buy_stop.expect("best trailing buy stop failed").borrow_mut()).get(price)
    T::get((*order_book).trailing_buy_stop, price)
}

// Method to get the trailing sell stop level
pub fn get_trailing_sell_stop_level<'a, B, R>(order_book: B, price: u64) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    T::get((*order_book).trailing_sell_stop, price)
}

pub fn get_next_trailing_stop_level<'a, B, R>(order_book: B, level_node: A) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>
{  
    if (*level_node.borrow_mut()).is_bid() {
        // Find the next level in reverse order in _trailing_sell_stop
        T::get_next_lower_level((*order_book).trailing_sell_stop.expect("best trailing sell stop failed"))
    } else {
        // Find the next level in normal order in _trailing_buy_stop
        T::get_next_higher_level((*order_book).trailing_buy_stop.expect("best trailing buy stop failed"))
    }
}

pub fn delete_trailing_stop_level<'a, B, R>(mut order_book: B, order: &Order<R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    // remove panicking behavior from code
    let level_node = order.level_node.expect("level node not retrieved");
    
    if order.is_buy() {
        // Update the best trailing buy stop order price level
        // remove panicking behavior from code
        let best_stop = (*order_book).best_trailing_buy_stop.expect("best stop not retrieved");
        let price: u64;
        if ptr::eq(&*best_stop, &*level_node) {
            let borrow_stop = (*best_stop).borrow();
            price = borrow_stop.level.price;
            (*order_book).best_trailing_buy_stop = if borrow_stop.right.is_none() {
                borrow_stop.right.clone()
            } else {
                borrow_stop.parent.clone()
            }
        }
        // Erase the price level from the trailing buy stop orders collection
        T::remove((*order_book).trailing_buy_stop, price);
    } else {
        // Update the best trailing sell stop order price level
        // remove panicking behavior from code
        let best_stop = (*order_book).best_trailing_sell_stop.expect("best stop not retrieved");
        let price: u64;
        if ptr::eq(&*best_stop, &*level_node) {
            let borrow_stop = (*best_stop).borrow();
            price = borrow_stop.level.price;
            (*order_book).best_trailing_sell_stop = if borrow_stop.left.is_none() {
                borrow_stop.left.clone()
            } else {
                borrow_stop.parent.clone()
            }
        }
        // Erase the price level from the trailing sell stop orders collection
        T::remove((*order_book).trailing_sell_stop, price);
    }
    // Release the price level
    // (*order_book).level_pool.release(level_node.price)
}

pub fn add_trailing_stop_level<'a, B>(mut order_book: B, order: &Order<R>) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    let (price, level_node) = if order.is_buy() {
        let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order.stop_price))));
        (order.stop_price, level_node)
    } else {
        let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order.stop_price))));
        (order.stop_price, level_node)
    };
    
    if order.is_buy() {
        (*order_book).trailing_buy_stop.insert(level_node);
        // Update the best trailing buy stop order price level
        if (*order_book).best_trailing_buy_stop.is_none() || ((*level_node).borrow().price < ((*order_book.best_trailing_buy_stop.expect("best trailing buy stop failed")).borrow()).price) {
            (*order_book).best_trailing_buy_stop = Some(level_node);
        }
    } else {
        (*order_book).trailing_sell_stop.insert(level_node);
        // Update the best trailing sell stop order price level
        if (*order_book).best_trailing_sell_stop.is_none() || ((*level_node).borrow().price < ((*order_book.best_trailing_sell_stop.expect("best trailing sell stop failed")).borrow()).price) {
            (*order_book).best_trailing_sell_stop = Some(level_node);
        }
    }
    Some(level_node)
}

pub fn best_buy_stop<'a, B>(order_book: B) -> Option<R> 
where 
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    (*order_book).best_buy_stop
}

// Method to get the best sell stop level
pub fn best_sell_stop<'a, B>(order_book: B) -> Option<R> 
where 
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    (*order_book).best_sell_stop
}

pub fn add_stop_level<'a, B>(mut order_book: B, order: &Order<R>) -> Option<R> 
where 
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    // Determine the level type and price based on the order node
    // Determine the price and create a level node
    let level_option = if order.is_buy() {
        Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order.stop_price))))
    } else {
        Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order.stop_price))))
    };

    let level_node = *level_option.borrow_mut();

    if order.is_buy() {
        (*order_book).buy_stop.insert(level_option);
        // remove panicking behavior from code
        let best_stop = (*order_book.best_buy_stop.expect("best stop")).borrow();
        if (*order_book).best_buy_stop.is_none() || (level_node.level.price < best_stop.level.price) {
            (*order_book).best_buy_stop = Some(level_option);
        }
    } else {
        (*order_book).sell_stop.insert(level_option);
        // remove panicking behavior from code
        let best_stop = (*order_book.best_sell_stop.expect("best stop")).borrow();
        if (*order_book).best_sell_stop.is_none() || (level_node.level.price < best_stop.level.price) {
            (*order_book).best_sell_stop = Some(level_option);
        }
    }
    Some(level_option)
}

pub fn create_and_insert_level<'a, T, B, E, O, H, R>(mut order_book: B, price: u64, level_type: LevelType) -> Option<R> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    // Create a new price level based on the provided level type
    // Insert the price level into the appropriate collection based on level type
    let new_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(level_type, price))));
    match level_type {
        LevelType::Bid => {
            if let Some(bids_root) = (*order_book).bids {
                T::insert(Rc::clone(&bids_root), Rc::clone(&new_node));
            } else {
                // Handle the case where bids tree is empty
                (*order_book).bids = Some(new_node);
            }
        },
        LevelType::Ask => {
            if let Some(asks_root) = order_book.asks {
                T::insert(Rc::clone(&asks_root), Rc::clone(&new_node));
            } else {
                // Handle the case where bids tree is empty
                (*order_book).asks = Some(new_node);
            }
        },
    }
    Some(new_node)
}


pub fn delete_level<'a, B, T, R>(mut order_book: B, order: &Order<R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    // remove panicking behavior from code
    let level_node = order.level_node.expect("order node level not retrieved");
    if order.is_buy() {
        // remove panicking behavior from code
        let best_bid = (*order_book).best_bid.expect("best bid not retrieved");
        let price: u64;
        if ptr::eq(&*best_bid, &*level_node) {
            // Update the best bid price level
            let borrowed_best = *best_bid.borrow_mut();
            (*order_book).best_bid = if borrowed_best.left.is_some() {
                borrowed_best.left
            } else if borrowed_best.parent.is_some() {
                borrowed_best.parent
            } else {
                borrowed_best.right
            };
            let price: u64 = (*(*order_book).bids.expect("asks not retrieved").borrow_mut()).price;
            T::remove((*order_book).asks, price);
        }
        // Erase the price level from the bid collection
    } else {
        // remove panicking behavior from code
        let best_ask: Rc<RefCell<LevelNode<'_>>> = (*order_book).best_ask.expect("best bid not retrieved");
        if ptr::eq(&*best_ask, &*level_node) {
            let borrowed_best = *best_ask.borrow_mut();
            // Update the best bid price level
            (*order_book).best_ask = if borrowed_best.left.is_some() {
                borrowed_best.left
            } else if borrowed_best.parent.is_some() {
                borrowed_best.parent
            } else {
                borrowed_best.right
            };
            let price: u64 = (*(*order_book).asks.expect("asks not retrieved").borrow_mut()).price;
            T::remove((*order_book).asks, price);
        }
    }
}

pub fn add_level<'a, T, B, E, O, H, R>(mut order_book: B, order: &Order<R>, tree: T) -> Option<R> 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    O: OrderOps<'a, B>,
    T: Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    B: MutableBook<'a>,
{
    let level_node = B::create_and_insert_level(order_book, order.price, if order.is_buy() { LevelType::Bid } else { LevelType::Ask }, tree);
    // remove panicking behavior from code
    let node_borrow = (*level_node).borrow();
    
    if order.is_buy() {
        // remove panicking behavior from code
        if (*order_book).best_bid.is_none() || (*node_borrow).price > (*(*order_book).best_bid.expect("best bid failed")).borrow().price {
            (*order_book).best_bid = Some(level_node)
        }
    } else {
        // remove panicking behavior from code
        if (*order_book).best_ask.is_none() || (*node_borrow).price < (*(*order_book).best_ask.expect("best ask failed")).borrow().price {
            (*order_book).best_ask = Some(level_node)
        }
    }
    Some(level_node)
}

pub fn get_next_level_node<'a, B, R>(order_book: B, level_node: A) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    todo!()
}

pub fn best_ask<'a, B, R>(order_book: B) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    (*order_book).best_ask
}

pub fn best_bid<'a, B, R>(order_book: B) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    (*order_book).best_bid
} 

pub fn get_bid<'a, B, T, R>(order_book: B, price: u64) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    let price: u64 = (*(*order_book).bids.expect("asks not retrieved").borrow_mut()).price;
    T::remove((*order_book).bids, price)
}

pub fn get_ask<'a, B, T, R>(order_book: B, price: u64) -> Option<R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    let price: u64 = (*(*order_book).asks.expect("asks not retrieved").borrow_mut()).price;
    T::remove((*order_book).asks, price)
}

pub fn get_market_trailing_stop_price_ask<'a, B>(order_book: B) -> u64
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{ 
    let last_price = (*order_book).last_ask_price;
    let best_price = (*order_book).best_ask.map_or(u64::MAX, |ask_node| (*ask_node).borrow().level.price);
    std::cmp::max(last_price, best_price)
}

pub fn get_market_trailing_stop_price_bid<'a, B>(order_book: B) -> u64 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    let last_price = (*order_book).last_bid_price;
    let best_price = if (*order_book).best_bid.is_some() {
        // remove panicking behavior from code
        (*order_book.best_bid.expect("best bid")).borrow().price
    } else {
        0
    };
    std::cmp::min(last_price, best_price)
}

pub fn is_top_of_book<'a, B>(order_book: B, order: &Order<R>) -> bool 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    if let Some(level_node) = order.level_node {
        return match order.is_buy() {
            true => {
                // remove panicking behavior from code
                (*order_book.best_bid.expect("best bid")).borrow().price == (*level_node).borrow().level.price
            },
            false => {
                // remove panicking behavior from code
                (*order_book.best_ask.expect("best ask")).borrow().price == (*level_node).borrow().level.price
            },
        };
    }
    false
}

pub fn update_level<'a, B, H>(order_book: B, update: LevelUpdate<'a, R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    H: Handler<'a, B>
{
    match update.update_type {
        UpdateType::Add => H::on_add_level(order_book, update.update_type, update.top),
        UpdateType::Update => H::on_update_level(order_book, update.update_type, update.top),
        UpdateType::Delete => H::on_delete_level(order_book, update.update_type, update.top),
        _ => return,
    };
    H::on_update_order_book(order_book, update.top)
}

pub fn on_trailing_stop<'a, B>(order_book: B, order: Order<'a, R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    // Here you would implement the specific logic for handling a trailing stop order
    // For example:
    if order.is_buy() {
        // Handle trailing stop for buy order
        // Update order book, prices, or other states as required
    } else {
        // Handle trailing stop for sell order
        // Update order book, prices, or other states as required
    }
    // Other logic as needed for trailing stops...
}

pub fn reset_matching_price<'a, B, R>(mut order_book: B) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    (*order_book).matching_bid_price = 0;
    (*order_book).matching_ask_price = u64::MAX;
}

pub fn get_market_ask_price<'a, B, R>(order_book: B) -> u64 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    let best_price = if (*order_book).best_ask.is_some() {
        // remove panicking behavior from code
        (*order_book.best_ask.expect("market ask price")).borrow().level.price
    } else {
        u64::MAX
    };
    min(best_price, (*order_book).matching_ask_price)
}

pub fn get_market_bid_price<'a, B, R>(order_book: B) -> u64 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    let best_price = if (*order_book).best_bid.is_some() {
        // remove panicking behavior from code
        (*order_book.best_bid.expect("market bid price")).borrow().level.price
    } else {
        0
    };
    max(best_price, (*order_book).matching_bid_price)
}

pub fn update_last_price<'a, B, R>(mut order_book: B, order: Order<R>, price: u64) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    if order.is_buy() {
        (*order_book).last_bid_price = price;
    } else {
        (*order_book).last_ask_price = price;
    }
}

pub fn update_matching_price<'a, B, R>(mut order_book: B, order: Order<R>, price: u64) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
{
    if order.is_buy() {
        (*order_book).matching_bid_price = price;
    } else {
        (*order_book).matching_ask_price = price;
    }
}

pub fn calculate_trailing_stop_price<'a, B, E, H, T, O, R>(order_book: B, order: Order<'a, R>) -> u64 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B>,
    B: MutableBook<'a>,
    T: Tree<'a, R>,
    O: OrderOps<'a, B>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    // Get the current market price
    let market_price = if order.is_buy() {
        B::get_market_trailing_stop_price_ask(order_book)
    } else {
        B::get_market_trailing_stop_price_bid(order_book)
    };
    let mut trailing_distance = order.trailing_distance as u64;
    let mut trailing_step = order.trailing_step as u64;

    // Convert percentage trailing values into absolute ones
    if trailing_distance < 0 {
        trailing_distance -= trailing_distance * market_price as u64 / 10000;
        trailing_step -= trailing_step * market_price as u64 / 10000;
    }

    let old_price = order.stop_price;

    if order.is_buy() {
        // Calculate a new stop price
        let new_price = market_price.checked_add(trailing_distance as u64).unwrap_or(u64::MAX);

        // If the new price is better and we get through the trailing step
        if new_price < old_price && (old_price - new_price) >= trailing_step as u64 {
            return new_price;
        }
    } else {
        // Calculate a new stop price
        let new_price = market_price.checked_sub(trailing_distance as u64).unwrap_or(0);

        // If the new price is better and we get through the trailing step
        if new_price > old_price && (new_price - old_price) >= trailing_step as u64 {
            return new_price;
        }
    }
    old_price
}

pub fn recalculate_trailing_stop_price<'a, H, T, O, B, E, R>(mut order_book: B, level: Level<R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    let mut new_trailing_price;

    // Skip recalculation if market price goes in the wrong direction
    match level.level_type {
        LevelType::Ask => {
            let old_trailing_price = (*order_book).trailing_ask_price;
            new_trailing_price = B::get_market_trailing_stop_price_ask(order_book);
            if new_trailing_price >= old_trailing_price {
                return;
            }
            (*order_book).trailing_ask_price = new_trailing_price;
        },
        LevelType::Bid => {
            let old_trailing_price = (*order_book).trailing_bid_price;
            new_trailing_price = B::get_market_trailing_stop_price_bid(order_book);
            if new_trailing_price <= old_trailing_price {
                return;
            }
            (*order_book).trailing_bid_price = new_trailing_price;
        },
    }

    // Recalculate trailing stop self.orders
    let mut current = match level.level_type {
        LevelType::Ask => {
            (*order_book).best_trailing_buy_stop
        },
        LevelType::Bid => {
            (*order_book).best_trailing_sell_stop
        }
    };

    let mut previous: Option<R> = None;

    while let Some(current_level) = current {
        let mut recalculated = false;
        let mut node = (*current_level).borrow_mut().orders.front_mut();

        while let Some(order) = node {
            let old_stop_price = order.stop_price;
            let new_stop_price = B::calculate_trailing_stop_price(order_book, *order);

            // Update and re-add order if stop price changed
            if new_stop_price != old_stop_price {
                B::delete_trailing_stop_order(order_book, &order);
                // Update stop price based on order type
                match order.order_type {
                    OrderType::TrailingStop => order.stop_price = new_stop_price,
                    OrderType::TrailingStopLimit => {
                        let diff = order.price - order.stop_price;
                        order.stop_price = new_stop_price;
                        order.price = new_stop_price + diff;
                    },
                    _ => panic!("Unsupported order type!"),
                }
                H::on_update_order(&order);
                B::add_trailing_stop_order(order_book, &order);
                recalculated = true;
            }
            let next_order = order.next_mut();
            node = next_order;
        }

        if recalculated {
            let current = if let Some(prev) = previous {
                Some(prev) 
            } else if level.level_type == LevelType::Ask {
                (*order_book).best_trailing_buy_stop
            } else {
                (*order_book).best_trailing_sell_stop
            };
        } else {
            previous = current;
            current = B::get_next_trailing_stop_level(order_book, current_level);
        }
    }
}


pub fn activate_stop_orders<'a, E, H, B, R>(mut order_book: B, mut orders: Orders< B>) -> bool 
where
    R: Ref<'a>,
    O: OrderOps<'a, B>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, H, M, A>,
    B: MutableBook<'a>,
{
    let mut result = false;
    let mut stop = false;

    while !stop {
        stop = true;

        // Try to activate buy stop self.orders
        if E::activate_individual_stop_orders(order_book, B::best_buy_stop(order_book), B::get_market_ask_price(order_book), orders)
            || E::activate_individual_stop_orders(order_book, B::best_trailing_buy_stop(order_book), B::get_market_ask_price(order_book), orders) {
            result = true;
            stop = false;
        }
        let best_ask = B::best_ask(order_book);
        
        // Recalculate trailing buy stop self.orders
        E::recalculate_trailing_stop_price(order_book, best_ask);

        // Try to activate sell stop self.orders
        if E::activate_individual_stop_orders(order_book, B::best_sell_stop(order_book), B::get_market_bid_price(order_book), orders)
            || E::activate_individual_stop_orders(order_book, B::best_trailing_sell_stop(order_book), B::get_market_bid_price(order_book), orders) {
            result = true;
            stop = false;
        }

        let best_bid = B::best_bid(order_book);
        // Recalculate trailing sell stop self.orders
        E::recalculate_trailing_stop_price(order_book, best_bid);
    
    }
    result
}

pub fn activate_individual_stop_orders<'a, E, C, R, H>(order_book: B, level_node: Option<R>, stop_price: u64, orders: Orders< B>) -> bool 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: MutableBook<'a>,
{

    let mut result = false;

    let mut borrowed_node = (*level_node.expect("level node borrow failed")).borrow_mut();
    let arbitrage = if borrowed_node.is_bid() {
        stop_price <= borrowed_node.level.price
    } else {
        stop_price >= borrowed_node.level.price
    };
    if !arbitrage {
        return false;
    }

    let mut activating_order = borrowed_node.orders.front_mut();

    while let Some(order) = activating_order {

        let mut next_activating_order = order.next_mut();

        match order.order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= E::activate_stop_order(order_book, order);
            },
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= E::activate_stop_limit_order(order_book, order);
            },
            _ => panic!("Unsupported order type!"),
        }
        activating_order = next_activating_order;
    }
    result
}

pub fn activate_stop_order<'a, E, O, A, H, T, B, M>(mut orders: Orders< B>, mut order_book: B, mut order: &Order<R>) -> bool 
where
    R: Ref<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    A: AsMut<T>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    B: MutableBook<'a>,
{
    // Delete the stop order from the order book
    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        B::delete_trailing_stop_order(order_book, &order);
    } else {
        B::delete_stop_order(order_book, order);
    }

    // Convert the stop order into the market order
    order.order_type = OrderType::Market;
    order.price = 0;
    order.stop_price = 0;
    order.time_in_force = if order.is_fok() { TimeInForce::FOK } else { TimeInForce::IOC };

    // Call the corresponding MarketHandler
    H::on_update_order(&order);

    // Match the market order
    E::match_market(order_book, order);

    // Call the corresponding MarketHandler
    H::on_delete_order(&order);

    // Erase the order
    O::remove_order(orders, &order.id);

    // Release the order, assuming we have an order pool with a release method
    // order_pool.release(order);
    true
}

pub fn activate_stop_limit_order<'a, E, A, H, B, M>(mut order_book: B, mut order: &Order<R>, mut orders: Orders< B>) -> bool 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, H, M, A>,
{
    // Delete the stop order from the order book
    if order.is_trailing_stop() || order.is_trailing_stop_limit() {
        B::delete_trailing_stop_order(order_book, &order);
    } else {
        B::delete_stop_order(order_book, order);
    }

    order.order_type = OrderType::Limit;
    order.stop_price = 0;

    H::on_update_order(&order);

    E::match_limit(order_book, &order);

    if order.leaves_quantity > 0 && !order.is_ioc() && !order.is_fok() {
        let level_update = B::add_order(order_book, order);
        E::update_level(order_book, level_update);
    } else {
        // Call the corresponding MarketHandler
        //H::on_delete_order(&order);
        O::remove_order(orders, &order.id);
        // order_pool.release(order);
    }
    true
}


pub fn add_order<'a, E, H,B, R>(order_book: B, order: &Order<R>) -> LevelUpdate<'a, R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    let mut update_type = UpdateType::Update;
    // Find the price level for the order
    let mut existing_level = if order.is_buy() {
        T::get((*order_book).bids, order.price)
      //  (*order_book.bids.expect("order book bids")).borrow_mut().get(&order.price)
    } else {
        T::get((*order_book).asks, order.price)
      //  (*order_book.asks.expect("order book asks")).borrow_mut().get(&order.price)
    };

    let binding: Option<R>;
    if let None = existing_level {
        binding = B::add_level(order_book, order);
        existing_level = binding;
        update_type = UpdateType::Add;
    }

    let level_node: Rc<RefCell<LevelNode<'_>>>;
    let mut level: Level<R>;

    if let Some(level_node) = existing_level {
        level = (*level_node).borrow().level;
        B::add_volumes(order_book, level, order);
        level.orders.push_back(*order);
        (*order.level_node.expect("order node level not obtained")).borrow().level = level;
    }

    LevelUpdate {
        update_type,
        update: Level { 
            level_type: level.level_type, 
            price: level.price, // Similarly for other fields
            total_volume: level.total_volume,
            hidden_volume: level.hidden_volume,
            visible_volume: level.visible_volume,
            orders: todo!(),
            _marker: std::marker::PhantomData,
        },
        top: B::is_top_of_book(order_book, order),
    }
}

pub fn add_limit_order<'a, E, H, B, OB, OC, R>(orders: Orders< B>, order: Order<R>, matching: bool, order_books: OBMap<'a, R, T>, recursive: bool) -> Result<(), ErrorCode> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
    OC: OrderBookContainer<'a, B, H, T>
{     
    let mut order_book = OC::get_order_book(order_books, &order.symbol_id).expect("order book not found");
    let order = E::get_order(orders, order.symbol_id).expect("order node not found");

    H::on_add_order(&order);

    if matching && !recursive {
        E::match_limit(order_book, &order);
    }

    if (order.leaves_quantity > 0) && !order.is_ioc() && !order.is_fok() {
    // let order = order.new(&order);
        if O::insert_order(orders, &order.id, order).is_some() {
            // Order duplicate
            H::on_delete_order(&order);
            // order_pool.release(order.new(&order));
        } else {
            // Update level with the new order
           // let order_book = B::add_order(order.new(&order));
            E::update_level(order_book, B::add_order(order_book, &Order::new(order)));
        }
    } else {
        H::on_delete_unmatched_order(&order);
    }

    if matching && !recursive {
        E::match_order_book::<H>(order_book);
    }

    B::reset_matching_price(order_book);

    Ok(())
}

pub fn add_stop_order<'a, T, E, H, O, B, R>(order_book: B, order: &Order<R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    // Find the price level for the order
    let level_node = if order.is_buy() {
        T::get((*order_book).buy_stop, order.stop_price)
       // (*B::buy_stop.borrow_mut()).get(&order.stop_price)
    } else {
        T::get((*order_book).sell_stop, order.stop_price)
       // (*B::sell_stop.borrow_mut()).get(&order.stop_price)
    };

    let binding = match level_node {
        Some(level) => level_node,
        None => {
            B::add_stop_level(order_book, order)
        },
    };

    if let Some(level_node) = binding {
        let mut level = (*level_node).borrow().level;
        B::add_volumes(order_book, level, order);
        // Link the new order to the orders list of the price level
        level.orders.push_back(*order); 
        (*order).level_node = Some(level_node)
    } else {
       // let level_node = (*level_node).borrow().level;
        (*order).level_node = level_node
    }
}

pub fn add_trailing_stop_order<'a, T, E, H, O, B, L, A, J, M>(order_book: B, order: &Order<R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    L: LevelOps<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    let level_node = if order.is_buy() {
        T::get((*order_book).trailing_buy_stop, order.stop_price)
            .or_else(|| {
                B::add_trailing_stop_level(order_book, order)
            })// Clones the Arc, not the Level
    } else {
        T::get((*order_book).trailing_buy_stop, order.stop_price)
            .or_else(|| {
                B::add_trailing_stop_level(order_book, order)
            }) // Clones the Arc, not the Level
    };

    let mut level = level_node.expect("tree operation failed").borrow_mut().level;
    // Update the price level volume
    L::add_volumes(&mut level, order);

    // Link the new order to the orders list of the price level
    // check for correctness
    L::link_order(&mut level, order);

    // Unlink the empty order from the orders list of the price level
    level.orders.push_back(*order);

    (*order.level_node.expect("order node level node expected")).borrow().level = level;
}


pub fn reduce_order<'a, T, E, H, O, B, R>(order_book: B, mut order: &'a Order<'a, R>, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate<'a, R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    let mut update_type = UpdateType::Update;
    let mut level_update: LevelUpdate;

    // remove panicking behavior from code
    let mut level_node = (*order.level_node.expect("level node not retrieved from order node").borrow_mut());
    let mut level = level_node.level;
    level.total_volume -= quantity;
    level.hidden_volume -= hidden;
    level.visible_volume -= visible;

    if order.leaves_quantity == 0 {
        //B::unlink_order(level, order)
        level.orders.pop_current(&order);
    }

    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_level(order_book, order);
        update_type = UpdateType::Delete;
    }
    
    LevelUpdate {
        update_type,
        update: Level { 
            level_type: level.level_type, 
            price: level.price, // Similarly for other fields
            total_volume: level.total_volume,
            hidden_volume: level.hidden_volume,
            visible_volume: level.visible_volume,
            orders: todo!(),
            _marker: std::marker::PhantomData,
        },
        top: B::is_top_of_book(order_book, order),
    }
}

pub fn delete_order<'a, T, E, H, O, B, L, G, R>(order_book: B, order: &'a Order<'a, R>) -> LevelUpdate<'a, R> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    E: Execution<'a, B, H>,
    A: Deref<Target = Order<'a, R>>,
    L: LevelOps<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    // remove panicking behavior from code
    let mut level_node = order.level_node.expect("level node not retrieved from order node");
    let mut level = (*level_node).borrow().level;
    
    // Update the price level volume
    L::subtract_volumes(&mut level, order);

    // Unlink the empty order from the orders list of the price level
    L::unlink_order(&mut level, order);

    let mut update_type = UpdateType::Update;
    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_level(order_book, order);
        update_type = UpdateType::Delete;
    }
    LevelUpdate {
        update_type,
        update: Level { 
            level_type: level.level_type, 
            price: level.price, // Similarly for other fields
            total_volume: level.total_volume,
            hidden_volume: level.hidden_volume,
            visible_volume: level.visible_volume,
            orders: todo!(),
            _marker: std::marker::PhantomData,
            
        },
        top: B::is_top_of_book(order_book, order),
    }

    
}

pub fn reduce_stop_order<'a, T, E, H, O, B, R>(order_book: B, order: &Order<R>, quantity: u64, hidden: u64, visible: u64) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    // Find the price level for the order
    // remove panicking behavior from code
    let mut level = (*order.level_node.expect("level node not retrieved from order node")).borrow().level;

    // Update the price level volume
    level.total_volume -= quantity;
    level.hidden_volume -= hidden;
    level.visible_volume -= visible;
    // Unlink the empty order from the orders list of the price level
    if order.leaves_quantity == 0 {
        // Assuming pop_current is a function that removes an order based on Some criteria and returns an Option<order /* Order */>
        level.orders.pop_current(&order);
    }
    // Delete the empty price level
    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_stop_level(order_book, order);
    };
}

pub fn delete_stop_order<'a, T, E, H, O, B, R>(order_book: B, order: &Order<R>) 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{    
    // Update the price level volume
    // Find the price level for the order
    // remove panicking behavior from code
    let mut level = (*order.level_node.expect("level node not retrieved from order node")).borrow().level;

    level.total_volume -= order.leaves_quantity();
    level.hidden_volume -= order.hidden_quantity();
    level.visible_volume -= order.visible_quantity();

    // Unlink the empty order from the orders list of the price level
    level.orders.pop_current(&order);

    // Delete the empty price level
    if level.total_volume == 0 {
        B::delete_stop_level(order_book, order);
    }
}


pub fn delete_trailing_stop_order<'a, E, T, H, O, B, G, L, R>(order_book: B, order: &Order<R>) -> Result<(), &'static str> 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    E: Execution<'a, B, H>,
    L: LevelOps<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    // remove panicking behavior from code
    let mut level = (*order.level_node.expect("level node not retrieved from order node")).borrow().level;
    
    // Update the price level volume
    // check for correctness with doubling up
    L::subtract_volumes(&mut level, order);

    // Unlink the empty order from the orders list of the price level
    // let mut level = (*level.expect("order node level node not found")).borrow().level;
    level.orders.pop_current(&order); // Assuming each order has a unique identifier

    // Delete the empty price level
    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_trailing_stop_level(order_book, order);
    };
    Ok(())
}

pub fn delete_stop_level<'a, B, R>(mut order_book: B, order: &Order<R>) 
where 
    R: Ref<'a>,
    B: MutableBook<'a>,
    T: Tree<'a, R>
{
    // remove panicking behavior from code
    let level_node = order.level_node.expect("order node level node not retrieved");

    if order.is_buy() {
        // Update the best buy stop order price level
        // remove panicking behavior from code
        let stop_level = (*order_book).best_buy_stop.expect("buy stop not found");
        let borrowed_level = *stop_level.borrow_mut();
        if ptr::eq(&*stop_level, &*level_node) {
            (*order_book).best_buy_stop = if borrowed_level.right.is_none() {
                borrowed_level.right
            } else {
                borrowed_level.parent
            }   
        }
        // Erase the price level from the buy stop orders collection
        T::remove((*order_book).best_buy_stop, borrowed_level.price);
       // (*stop_level.borrow_mut()).remove(borrowed_level.price);
    } else {
        // remove panicking behavior from code
        let stop_level = (*order_book).best_sell_stop.expect("buy stop not found");
        let borrowed_level = *stop_level.borrow_mut();
        if ptr::eq(&*stop_level, &*level_node)  {
            // Update the best sell stop order price level
            (*order_book).best_sell_stop = if borrowed_level.right.is_none() {
                borrowed_level.right
            } else {
                borrowed_level.parent
            }
        }
        // Erase the price level from the sell stop orders collection
        T::remove((*order_book).best_sell_stop, borrowed_level.price);
       // (*stop_level.borrow_mut()).remove(borrowed_level.price);
    }
}


pub fn activate_stop_orders_level<'a, E, T, H, O, B, R>(order_book: B, mut level: Level<'a, R>, stop_price: u64) -> bool 
where
    R: Ref<'a>,
    B: MutableBook<'a>,
    E: Execution<'a, B, H>,
    H: Handler<'a, B> + OrderOps<'a, B> + Tree<'a, R>,
    B: OrderBookOperations<'a, E, O, H, T, M, A>,
{
    let mut result = false;
    
    let arbitrage = if level.is_bid() {
        stop_price <= level.price
    } else {
        stop_price >= level.price
    };

    if !arbitrage {
        return false;
    }

    let mut activating_order = level.orders.front_mut();
    while let Some(order) = activating_order {
        // Clone next_order to avoid borrow_muting issues
        let next_activating_order = order.next_mut();

        match order.order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= E::activate_stop_order(order_book, order);
            }
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= E::activate_stop_limit_order(order_book, order);
            }
            _ => {
                assert!(false, "Unsupported order type!");
            }
        }
        //let next_order = next_activating_order;
        activating_order = next_activating_order;
    }
    result
}
