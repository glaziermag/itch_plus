
use std::{ops::Deref, rc::Rc, cell::RefCell, ptr, cmp::{max, min}};
use crate::{levels::{level::{UpdateType, LevelUpdate, Level, LevelType}, indexing::{RcNode, LevelNode, Tree}}, orders::{order::{OrderNode, Order, ErrorCode, TimeInForce, OrderType}, orders::OrderOps}, market_handler::Handler, market_executors::{executor::Execution, order_book_operations::OrderBooks}};

#[derive(Debug)]
pub enum OrderBookError {
    OrderNotFound,
    LevelNotFound,
}

// Trait defining operations on an OrderBook
pub trait OrderBookOperations<'a, C: Deref<Target = OrderBook<'a>>, E: Execution<'a>, O: OrderOps, H: Handler, T: Tree<'a>> {
    fn activate_stop_orders_level(order_book: C, level: Level, stop_price: u64) -> bool;
    fn delete_stop_level(order_book: C, order_node: &OrderNode);
    fn add_stop_level(order_book: C, order_node: &OrderNode) -> Option<RcNode<'a>>;
    fn best_sell_stop(order_book: C) -> Option<RcNode<'a>>;
    fn best_buy_stop(order_book: C) -> Option<RcNode<'a>>;
    fn reset_matching_price(order_book: C);
    fn get_market_ask_price(order_book: C) -> u64;
    fn get_market_bid_price(order_book: C) -> u64;
    fn update_last_price(order_book: C, order: Order, price: u64);
    fn update_matching_price(order_book: C, order: Order, price: u64);
    fn calculate_trailing_stop_price(order_book: C, order: Order) -> u64;
    fn recalculate_trailing_stop_price(order_book: C, level: Level);
    fn add_limit_order(orders: O, order: Order, matching: bool, order_books: OrderBooks, recursive: bool) -> Result<(), ErrorCode>;
    fn add_order(order_book: C, order_node: &OrderNode) -> LevelUpdate<'a>;
    fn add_stop_order(order_book: C, order_node: &OrderNode);
    fn add_trailing_stop_order(order_book: C, order_node: &OrderNode);
    fn reduce_order(order_book: C, order_node: &'a OrderNode<'a>, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate<'a>; 
    fn delete_order(order_book: C, order_node: &'a OrderNode<'a>) -> LevelUpdate<'a>;
    fn reduce_trailing_stop_order(order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64);
    fn delete_trailing_stop_order(order_book: C, order_node: &OrderNode) -> Result<(), &'static str>;
    fn get_market_trailing_stop_price_ask(order_book: C);
    fn get_market_trailing_stop_price_bid(order_book: C);
    fn reduce_stop_order(order_book: C, order_node: &OrderNode<'a>, quantity: u64, hidden_delta: u64, visible_delta: u64);
    fn get_next_level_node(order_book: C, level_node: &LevelNode<'a>) -> Option<LevelNode<'a>>;
    fn best_trailing_buy_stop(order_book: C) -> Option<LevelNode<'a>>;
    fn best_trailing_sell_stop(order_book: C) -> Option<LevelNode<'a>>;
    fn delete_trailing_stop_level(order_book: C, order_node: &OrderNode<'a>);
    fn activate_individual_stop_orders(order_book: C, level_node: Option<LevelNode<'a>>, market_price: u64, orders: O) -> bool;
    fn create_and_insert_level(order_book: C, price: u64, level_type: LevelType, tree: T) -> RcNode<'a>;
    fn best_ask(order_book: C) -> Option<RcNode<'a>>;
    fn best_bid(order_book: C) -> Option<RcNode<'a>>;
    fn delete_level(order_book: C, order_node: &OrderNode);
    fn is_top_of_book(order_book: C, order_node: &OrderNode) -> bool;
    fn subtract_level_volumes(order_book: C, level: RcNode<'a>, order_node: &OrderNode);
    fn unlink_order(order_book: C, level: RcNode<'a>, order_node: &OrderNode);
    fn delete_stop_order(order_book: C, order_node: &OrderNode); 
    fn add_level_volumes(order_book: C, level: RcNode<'a>, order_node: &OrderNode);
}

impl<'a, C: Deref<Target = OrderBook<'a>>, E: Execution<'a>, O: OrderOps, H: Handler, T: Tree<'a>> OrderBookOperations<'a, C, E, O, H, T> for OrderBook<'a> {
    fn add_level_volumes(order_book: C, level: RcNode<'a>, order_node: &OrderNode) {
        todo!()
    }
    fn delete_stop_order(order_book: C, order_node: &OrderNode) {
        todo!()
    }
    
    fn get_market_trailing_stop_price_ask(order_book: C) {
        todo!()
    }

    fn get_market_trailing_stop_price_bid(order_book: C){
        todo!()
    }

    fn activate_stop_orders_level(order_book: C, level: Level, stop_price: u64) -> bool {
        todo!()
    }

    fn delete_stop_level(order_book: C, order_node: &OrderNode) {
        todo!()
    }

    fn add_stop_level(order_book: C, order_node: &OrderNode) -> Option<RcNode<'a>> {
        todo!()
    }

    fn best_sell_stop(order_book: C) -> Option<RcNode<'a>> {
        todo!()
    }

    fn best_buy_stop(order_book: C) -> Option<RcNode<'a>> {
        todo!()
    }

    fn reset_matching_price(order_book: C) {
        todo!()
    }

    fn get_market_ask_price(order_book: C) -> u64 {
        todo!()
    }

    fn get_market_bid_price(order_book: C) -> u64 {
        todo!()
    }

    fn update_last_price(order_book: C, order: Order, price: u64) {
        todo!()
    }

    fn update_matching_price(order_book: C, order: Order, price: u64) {
        todo!()
    }

    fn calculate_trailing_stop_price(order_book: C, order: Order) -> u64 {
        todo!()
    }

    fn recalculate_trailing_stop_price(order_book: C, level: Level) {
        todo!()
    }

    fn add_limit_order(orders: O, order: Order, matching: bool, order_books: OrderBooks, recursive: bool) -> Result<(), ErrorCode> {
        todo!()
    }

    fn add_order(order_book: C, order_node: &OrderNode) -> LevelUpdate<'a> {
        todo!()
    }

    fn add_stop_order(order_book: C, order_node: &OrderNode) {
        todo!()
    }

    fn add_trailing_stop_order(order_book: C, order_node: &OrderNode) {
        todo!()
    }

    fn reduce_order(order_book: C, order_node: &'a OrderNode<'a>, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate<'a> {
        todo!()
    }

    fn delete_order(order_book: C, order_node: &'a OrderNode<'a>) -> LevelUpdate<'a> {
        todo!()
    }

    fn reduce_trailing_stop_order(order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) {
        todo!()
    }

    fn delete_trailing_stop_order(order_book: C, order_node: &OrderNode) -> Result<(), &'static str> {
        todo!()
    }

    fn reduce_stop_order(order_book: C, order_node: &OrderNode<'a>, quantity: u64, hidden_delta: u64, visible_delta: u64) {
        todo!(); // Replace with actual logic
    }

    fn delete_trailing_stop_level(order_book: C, order_node: &OrderNode<'a>) {
        todo!(); // Replace with actual logic
    }

    fn get_next_level_node(order_book: C, level_node: &LevelNode<'a>) -> Option<LevelNode<'a>> {
        todo!()
    }

    fn best_trailing_buy_stop(order_book: C) -> Option<LevelNode<'a>> {
        todo!()
    }

    fn best_trailing_sell_stop(order_book: C) -> Option<LevelNode<'a>> {
        todo!()
    }

    fn activate_individual_stop_orders(order_book: C, level_node: Option<LevelNode<'a>>, market_price: u64, orders: O) -> bool {
        todo!()
    }
    fn create_and_insert_level(order_book: C, price: u64, level_type: LevelType, tree: T) -> RcNode<'a> {
        // Placeholder: Implement the logic to create and insert a level
        todo!()
    }

    fn best_ask(order_book: C) -> Option<RcNode<'a>> {
        // Placeholder: Return the best ask level
        todo!()
    }

    fn best_bid(order_book: C) -> Option<RcNode<'a>> {
        // Placeholder: Return the best bid level
        todo!()
    }

    fn delete_level(order_book: C, order_node: &OrderNode) {
        // Placeholder: Delete a level based on the order node
        todo!()
    }

    fn is_top_of_book(order_book: C, order_node: &OrderNode) -> bool {
        // Placeholder: Determine if an order node is at the top of the book
        todo!()
    }

    fn subtract_level_volumes(order_book: C, level: RcNode<'a>, order_node: &OrderNode) {
        // Placeholder: Subtract volumes from a level
        todo!()
    }

    fn unlink_order(order_book: C, level: RcNode<'a>, order_node: &OrderNode) {
        // Placeholder:
    }
}

#[derive(Default)]
pub struct OrderBook<'a> {

    pub best_bid: Option<RcNode<'a>>,
    pub best_ask: Option<RcNode<'a>>,
    pub bids: Option<RcNode<'a>>,
    pub asks: Option<RcNode<'a>>,

    pub best_buy_stop: Option<RcNode<'a>>,
    pub best_sell_stop: Option<RcNode<'a>>,
    pub buy_stop: Option<RcNode<'a>>,
    pub sell_stop: Option<RcNode<'a>>,

    pub(crate) last_bid_price: u64,
    pub(crate) last_ask_price: u64,
    pub(crate) matching_bid_price: u64,
    pub(crate) matching_ask_price: u64,

    pub best_trailing_buy_stop: Option<RcNode<'a>>,
    pub best_trailing_sell_stop: Option<RcNode<'a>>,
    pub trailing_buy_stop: Option<RcNode<'a>>,
    pub trailing_sell_stop: Option<RcNode<'a>>,
    pub trailing_bid_price: u64,
    pub trailing_ask_price: u64,
}

impl<'a> OrderBook<'_> {

    pub fn new() -> OrderBook<'a> {
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
        }
    }
}

pub fn reduce_trailing_stop_order<'a, E: Execution<'a>>(order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) {
    // Assuming we have a way to get a mutable reference to an order and its level.
    // Update the price level volume
    // remove panicking behavior from code
    let mut level_node = order_node.level_node.expect("level node not retrieved from order node");
    let level = (*level_node.borrow()).level;
    // Update the price level volume
    let level_node = Some(level_node);
    E::subtract_level_volumes(level_node, order_node);
    // Unlink the empty order from the orders list of the price level
    if order_node.order.leaves_quantity == 0 {
        E::unlink_order(level_node, *order_node);
    }
    // Delete the empty price level
    if level.total_volume == 0 {
        order_node.level_node = None
    }
}

// Method to get the best trailing buy stop level
pub fn best_trailing_buy_stop<'a, C>(order_book: C) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    (*order_book).best_trailing_buy_stop
}

// Method to get the best trailing sell stop level
pub fn best_trailing_sell_stop<'a, C>(order_book: C) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    (*order_book).best_trailing_sell_stop
}

pub fn get_trailing_buy_stop_level<'a, C>(order_book: C, price: u64) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    ((*order_book).trailing_buy_stop.expect("best trailing buy stop failed").borrow_mut()).get(price)
}

// Method to get the trailing sell stop level
pub fn get_trailing_sell_stop_level<'a, C>(order_book: C, price: u64) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    ((*order_book).trailing_sell_stop.expect("best trailing sell stop failed").borrow_mut()).get(price)
}

pub fn get_next_trailing_stop_level<'a, C, T: Tree<'a>>(order_book: C, level_node: RcNode) -> Option<RcNode> 
where
    C: Deref<Target = OrderBook<'a>>,
{  
    if (*level_node.borrow_mut()).is_bid() {
        // Find the next level in reverse order in _trailing_sell_stop
        <LevelNode as Tree>::get_next_lower_level((*order_book).trailing_sell_stop.expect("best trailing sell stop failed"))
    } else {
        // Find the next level in normal order in _trailing_buy_stop
        <LevelNode as Tree>::get_next_higher_level((*order_book).trailing_buy_stop.expect("best trailing buy stop failed"))
    }
}

pub fn delete_trailing_stop_level<'a, C>(mut order_book: C, order_node: &OrderNode) 
where
    C: Deref<Target = OrderBook<'a>>,
{
    // remove panicking behavior from code
    let level_node = order_node.level_node.expect("level node not retrieved");
    
    if order_node.is_buy() {
        // Update the best trailing buy stop order price level
        // remove panicking behavior from code
        let best_stop = (*order_book).best_trailing_buy_stop.expect("best stop not retrieved");
        if ptr::eq(&*best_stop, &*level_node) {
            let borrow_stop = best_stop.borrow();
            (*order_book).best_trailing_buy_stop = if borrow_stop.right.is_none() {
                borrow_stop.right.clone()
            } else {
                borrow_stop.parent.clone()
            }
        }
        // Erase the price level from the trailing buy stop orders collection
        ((*order_book).trailing_buy_stop.expect("best trailing buy stop failed").borrow_mut()).remove((*level_node.borrow()).price);
    } else {
        // Update the best trailing sell stop order price level
        // remove panicking behavior from code
        let best_stop = (*order_book).best_trailing_sell_stop.expect("best stop not retrieved");
        if ptr::eq(&*best_stop, &*level_node) {
            let borrow_stop = best_stop.borrow();
            (*order_book).best_trailing_sell_stop = if borrow_stop.left.is_none() {
                borrow_stop.left.clone()
            } else {
                borrow_stop.parent.clone()
            }
        }
        // Erase the price level from the trailing sell stop orders collection
        ((*order_book).trailing_sell_stop.expect("best trailing sell stop failed").borrow_mut()).remove((*level_node.borrow()).price);
    }
    // Release the price level
    // (*order_book).level_pool.release(level_node.price)
}

pub fn add_trailing_stop_level<'a, C>(mut order_book: C, order_node: &OrderNode) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    let (price, level_node) = if order_node.is_buy() {
        let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order_node.stop_price))));
        (order_node.stop_price, level_node)
    } else {
        let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order_node.stop_price))));
        (order_node.stop_price, level_node)
    };
    
    if order_node.is_buy() {
        (*order_book).trailing_buy_stop.insert(level_node);
        // Update the best trailing buy stop order price level
        if (*order_book).best_trailing_buy_stop.is_none() || ((*level_node.borrow()).price < ((*order_book).best_trailing_buy_stop.expect("best trailing sell stop failed").borrow()).price) {
            (*order_book).best_trailing_buy_stop = Some(level_node);
        }
    } else {
        (*order_book).trailing_sell_stop.insert(level_node);
        // Update the best trailing sell stop order price level
        if (*order_book).best_trailing_sell_stop.is_none() || ((*level_node.borrow()).price < ((*order_book).best_trailing_sell_stop.expect("best trailing sell stop failed").borrow()).price) {
            (*order_book).best_trailing_sell_stop = Some(level_node);
        }
    }
    Some(level_node)
}

pub fn best_buy_stop<'a, C>(order_book: C) -> Option<RcNode<'a>> 
where 
    C: Deref<Target = OrderBook<'a>>
{
    (*order_book).best_buy_stop
}

// Method to get the best sell stop level
pub fn best_sell_stop<'a, C>(order_book: C) -> Option<RcNode<'a>> 
where 
    C: Deref<Target = OrderBook<'a>>
{
    (*order_book).best_sell_stop
}

pub fn add_stop_level<'a, C>(mut order_book: C, order_node: &OrderNode) -> Option<RcNode<'a>> 
where 
    C: Deref<Target = OrderBook<'a>>
{
    // Determine the level type and price based on the order node
    // Determine the price and create a level node
    let level_option = if order_node.is_buy() {
        Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order_node.stop_price))))
    } else {
        Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order_node.stop_price))))
    };

    let level_node = *level_option.borrow_mut();

    if order_node.is_buy() {
        (*order_book).buy_stop.insert(level_option);
        // remove panicking behavior from code
        let best_stop = (*order_book).best_buy_stop.expect("best stop").borrow();
        if (*order_book).best_buy_stop.is_none() || (level_node.level.price < best_stop.level.price) {
            (*order_book).best_buy_stop = Some(level_option);
        }
    } else {
        (*order_book).sell_stop.insert(level_option);
        // remove panicking behavior from code
        let best_stop = (*order_book).best_buy_stop.expect("best stop").borrow();
        if (*order_book).best_sell_stop.is_none() || (level_node.level.price < best_stop.level.price) {
            (*order_book).best_sell_stop = Some(level_option);
        }
    }
    Some(level_option)
}

pub fn create_and_insert_level<'a, T, C, B, E, O, H>(mut order_book: C, price: u64, level_type: LevelType, tree: T) -> Option<Rc<RefCell<LevelNode<'a>>>> 
where
    T: Tree<'a>,
    C: Deref<Target = OrderBook<'a>>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
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


pub fn delete_level<'a, C, B>(mut order_book: C, order_node: &OrderNode) 
where
    C: Deref<Target = OrderBook<'a>>,
    
{
    // remove panicking behavior from code
    let level_node = order_node.level_node.expect("order node level not retrieved");
    if order_node.is_buy() {
        // remove panicking behavior from code
        let best_bid = (*order_book).best_bid.expect("best bid not retrieved");
        let borrowed_best = *best_bid.borrow_mut();
        if ptr::eq(&*best_bid, &*level_node) {
            // Update the best bid price level
            (*order_book).best_bid = if borrowed_best.left.is_some() {
                borrowed_best.left
            } else if borrowed_best.parent.is_some() {
                borrowed_best.parent
            } else {
                borrowed_best.right
            };
            (*(*order_book).bids.expect("bids not retrieved").borrow_mut()).remove((*level_node.borrow_mut()).price);
        }
        // Erase the price level from the bid collection
    } else {
        // remove panicking behavior from code
        let best_ask: Rc<RefCell<LevelNode<'_>>> = (*order_book).best_ask.expect("best bid not retrieved");
        let borrowed_best = *best_ask.borrow_mut();
        if ptr::eq(&*best_ask, &*level_node) {
            // Update the best bid price level
            (*order_book).best_ask = if borrowed_best.left.is_some() {
                borrowed_best.left
            } else if borrowed_best.parent.is_some() {
                borrowed_best.parent
            } else {
                borrowed_best.right
            };
            (*(*order_book).asks.expect("asks not retrieved").borrow_mut()).remove((*level_node.borrow_mut()).price);
        }
    }
}

pub fn add_level<'a, T, C, B, E, O, H>(mut order_book: C, order_node: &OrderNode, tree: T) -> Option<RcNode<'a>> 
where
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    T: Tree<'a>,
    B: OrderBookOperations<'a, C, E, O, H, T>,
    C: Deref<Target = OrderBook<'a>>,
{
    let level_node = B::create_and_insert_level(order_book, order_node.price, if order_node.is_buy() { LevelType::Bid } else { LevelType::Ask }, tree);
    // remove panicking behavior from code
    let node_borrow = level_node.borrow();
    
    if order_node.is_buy() {
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

pub fn get_next_level_node<'a, C, B>(order_book: C, level_node: &LevelNode<'a>) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    todo!()
}

pub fn best_ask<'a, C, B>(order_book: C) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    (*order_book).best_ask
}

pub fn best_bid<'a, C, B>(order_book: C) -> Option<RcNode<'a>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    (*order_book).best_bid
} 

pub fn get_bid<'a, C, B>(order_book: C, price: u64) -> Option<Rc<RefCell<LevelNode<'a>>>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    (*(*order_book).bids.expect("bids not retrieved during get").borrow_mut()).get(price)
}

pub fn get_ask<'a, C, B>(order_book: C, price: u64) -> Option<Rc<RefCell<LevelNode<'a>>>> 
where
    C: Deref<Target = OrderBook<'a>>,
{
    (*(*order_book).asks.expect("asks not retrieved during get").borrow_mut()).get(price)
}



pub fn get_market_trailing_stop_price_ask<'a, C>(order_book: C) -> u64
where
    C: Deref<Target = OrderBook<'a>>,
{ 
    let last_price = (*order_book).last_ask_price;
    let best_price = (*order_book).best_ask.map_or(u64::MAX, |ask_node| ask_node.node.value);
    std::cmp::max(last_price, best_price)
}

pub fn get_market_trailing_stop_price_bid<'a, C>(order_book: C) -> u64 
where
    C: Deref<Target = OrderBook<'a>>,
{
    let last_price = (*order_book).last_bid_price;
    let best_price = if (*order_book).best_bid.is_some() {
        // remove panicking behavior from code
        (*order_book).best_bid.expect("best bid").price
    } else {
        0
    };
    std::cmp::min(last_price, best_price)
}

pub fn is_top_of_book<'a, C>(order_book: C, order_node: &OrderNode) -> bool 
where
    C: Deref<Target = OrderBook<'a>>,
{
    if let level = order_node.level_node.level {
        return match order_node.is_buy() {
            true => {
                // remove panicking behavior from code
                (*order_book).best_bid.expect("best bid").price == level.price
            },
            false => {
                let best_ask = (*order_book).best_ask;
                // remove panicking behavior from code
                (*order_book).best_bid.expect("best bid").price == level.price
            },
        };
    }
    false
}

pub fn update_level<'a, H: Handler, C: Deref<Target = OrderBook<'a>>>(order_book: C, update: LevelUpdate) 
where
    C: Deref<Target = OrderBook<'a>>,
{
    match update.update_type {
        UpdateType::Add => H::on_add_level(order_book, &update.update, update.top),
        UpdateType::Update => H::on_update_level(order_book, &update.update, update.top),
        UpdateType::Delete => H::on_delete_level(order_book, &update.update, update.top),
        _ => return,
    };
    H::on_update_order_book(order_book, update.top)
}

pub fn on_trailing_stop<'a, C>(order_book: C, order: Order) 
where
    C: Deref<Target = OrderBook<'a>>,
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

pub fn reset_matching_price<'a, C, B>(mut order_book: C) 
where
    C: Deref<Target = OrderBook<'a>>
{
    (*order_book).matching_bid_price = 0;
    (*order_book).matching_ask_price = u64::MAX;
}

pub fn get_market_ask_price<'a, C, B>(order_book: C) -> u64 
where
    C: Deref<Target = OrderBook<'a>>,
{
    let best_price = if (*order_book).best_ask.is_some() {
        // remove panicking behavior from code
        (*order_book).best_ask.expect("market ask price").price
    } else {
        u64::MAX
    };
    min(best_price, (*order_book).matching_ask_price)
}

pub fn get_market_bid_price<'a, C, B>(order_book: C) -> u64 
where
    C: Deref<Target = OrderBook<'a>>,
{
    let best_price = if (*order_book).best_bid.is_some() {
        // remove panicking behavior from code
        (*order_book).best_bid.expect("market bid price").price
    } else {
        0
    };
    max(best_price, (*order_book).matching_bid_price)
}

pub fn update_last_price<'a, C, B>(mut order_book: C, order: Order, price: u64) 
where
    C: Deref<Target = OrderBook<'a>>,
{
    if order.is_buy() {
        (*order_book).last_bid_price = price;
    } else {
        (*order_book).last_ask_price = price;
    }
}

pub fn update_matching_price<'a, C, B>(mut order_book: C, order: Order, price: u64) 
where
    C: Deref<Target = OrderBook<'a>>,
{
    if order.is_buy() {
        (*order_book).matching_bid_price = price;
    } else {
        (*order_book).matching_ask_price = price;
    }
}

pub fn calculate_trailing_stop_price<'a, C, B, E, H, T, O>(order_book: C, order: Order) -> u64 
where
    E: Execution<'a>,
    H: Handler,
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
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

pub fn recalculate_trailing_stop_price<'a, C, H, T, O, B, E>(order_book: C, level: Level) 
where
    C: Deref<Target = OrderBook<'a>>,
    E: Execution<'a>,
    H: Handler,
    T: Tree<'a>,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
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

    let mut previous: Option<LevelNode> = None;

    while let Some(current_level) = current {
        let mut recalculated = false;
        let mut node = current_level.orders.front_mut();

        while let Some(order_node) = node {
            let old_stop_price = order_node.stop_price;
            let new_stop_price = B::calculate_trailing_stop_price(order_book, order_node.order);

            // Update and re-add order if stop price changed
            if new_stop_price != old_stop_price {
                B::delete_trailing_stop_order(order_book, order_node);
                // Update stop price based on order type
                match order_node.order_type {
                    OrderType::TrailingStop => order_node.stop_price = new_stop_price,
                    OrderType::TrailingStopLimit => {
                        let diff = order_node.price - order_node.stop_price;
                        order_node.stop_price = new_stop_price;
                        order_node.price = new_stop_price + diff;
                    },
                    _ => panic!("Unsupported order type!"),
                }
                H::on_update_order(&order_node.order);
                B::add_trailing_stop_order(order_book, order_node);
                recalculated = true;
            }
            let next_order = order_node.next_mut();
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
            current = Some(B::get_next_trailing_stop_level(order_book, current_level));
        }
    }
}


pub fn activate_stop_orders<'a, E, O, C, T, H, B>(mut order_book: C, mut orders: O) -> bool 
where
    O: OrderOps,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    T: Tree<'a>,
    B: OrderBookOperations<'a, C, E, O, H, T>,
    C: Deref<Target = OrderBook<'a>>,
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

pub fn activate_individual_stop_orders<'a, E, O, A, C>(order_book: C, level_node: A, stop_price: u64, orders: O) -> bool 
where
    E: Execution<'a>,
    O: OrderOps,
    A: AsMut<RcNode<'a>>,
    C: Deref<Target = OrderBook<'a>>
{

    let mut result = false;

    let arbitrage = if level_node.is_bid() {
        stop_price <= level_node.price
    } else {
        stop_price >= level_node.price
    };
    if !arbitrage {
        return false;
    }

    let mut activating_order_node = level_node.orders.front_mut();

    while let Some(order_node) = activating_order_node {

        let mut next_activating_order_node = order_node.next_mut();

        match order_node.order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= E::activate_stop_order(order_book, order_node);
            },
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= E::activate_stop_limit_order(order_book, order_node);
            },
            _ => panic!("Unsupported order type!"),
        }
        activating_order_node = next_activating_order_node;
    }
    result
}

pub fn activate_stop_order<'a, E, O, A, C, H, T, B>(orders: O, mut order_book: C, mut order_node: &OrderNode) -> bool 
where
    O: OrderOps,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    T: Tree<'a>,
    A: AsMut<RcNode<'a>>,
    B: OrderBookOperations<'a, C, E, O, H, T>,
    C: Deref<Target = OrderBook<'a>>,
{
    // Delete the stop order from the order book
    if order_node.is_trailing_stop() || order_node.is_trailing_stop_limit() {
        B::delete_trailing_stop_order(order_book, order_node);
    } else {
        B::delete_stop_order(order_book, order_node);
    }

    // Convert the stop order into the market order
    order_node.order_type = OrderType::Market;
    order_node.price = 0;
    order_node.stop_price = 0;
    order_node.time_in_force = if order_node.is_fok() { TimeInForce::FOK } else { TimeInForce::IOC };

    // Call the corresponding MarketHandler
    H::on_update_order(&order_node.order);

    // Match the market order
    E::match_market(order_book, order_node.order);

    // Call the corresponding MarketHandler
    H::on_delete_order_node(order_node);

    // Erase the order
    orders.remove_order(&order_node.id);

    // Release the order, assuming we have an order pool with a release method
    // order_pool.release(order_node);
    true
}

pub fn activate_stop_limit_order<'a, E, O, A, C, H, B, T>(mut order_book: C, mut order_node: &OrderNode, mut orders: O) -> bool 
where
    T: Tree<'a>,
    C: Deref<Target = OrderBook<'a>>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // Delete the stop order from the order book
    if order_node.is_trailing_stop() || order_node.is_trailing_stop_limit() {
        B::delete_trailing_stop_order(order_book, order_node);
    } else {
        B::delete_stop_order(order_book, order_node);
    }

    order_node.order_type = OrderType::Limit;
    order_node.stop_price = 0;

    H::on_update_order(&order_node.order);

    E::match_limit(order_book, order_node.order);

    if order_node.order.leaves_quantity > 0 && !order_node.is_ioc() && !order_node.is_fok() {
        let level_update = B::add_order(order_book, order_node);
        E::update_level(order_book, level_update);
    } else {
        // Call the corresponding MarketHandler
        //H::on_delete_order(&order_node.order);
        orders.remove_order(&order_node.order.id);
        // order_pool.release(order_node);
    }
    true
}


pub fn add_order<'a, T, C, E, H, O, B>(order_book: C, order_node: &OrderNode) -> LevelUpdate<'a> 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{

    let mut update_type = UpdateType::Update;
    // Find the price level for the order
    let mut existing_level = if order_node.is_buy() {
        (*order_book.bids.borrow_mut()).get(&order_node.order.price)
    } else {
        (*order_book.asks.borrow_mut()).get(&order_node.order.price)
    };

    let binding: Level;
    if let None = existing_level {
        binding = B::add_level(order_book, order_node);
        existing_level = Some(&binding);
        update_type = UpdateType::Add;
    }

    let level: Level = Default::default();

    if let Some(level) = existing_level {
        let mut level = level.level;
        B::add_level_volumes(order_book, level, order_node);
        level.orders.push_back(order_node.clone());
        level.orders += 1;
        order_node.level = level;
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
            tree_node: todo!(),
        },
        top: B::is_top_of_book(order_book, order_node),
    }
}

pub fn add_limit_order<'a, E, H, O, C, T, B>(orders: O, order: Order, matching: bool, order_books: OrderBooks, recursive: bool) -> Result<(), ErrorCode> 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{     
    let mut order_book = order_books.get_order_book(&order.symbol_id);
    let order_node = E::get_order_node(order.symbol_id);

    H::on_add_order(&order);

    if matching && !recursive {
        E::match_limit(order_book, order);
    }

    if (order.leaves_quantity > 0) && !order.is_ioc() && !order.is_fok() {
    // let order_node = order_node.new(&order);
        if orders.insert(order_node.new(&order).id, order_node.new(&order)).is_some() {
            // Order duplicate
            H::on_delete_order(order_node);
            // order_pool.release(order_node.new(&order));
        } else {
            // Update level with the new order
           // let order_book = B::add_order(order_node.new(&order));
            E::update_level(order_book, B::add_order(order_book, order_node.new(&order)));
        }
    } else {
        H::on_delete_unmatched_order(&order);
    }

    if matching && !recursive {
        E::match_order_book(order_book);
    }

    B::reset_matching_price(order_book);

    Ok(())
}

pub fn add_stop_order<'a, C, T, E, H, O, B>(order_book: C, order_node: &OrderNode) 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // Find the price level for the order
    let level = if order_node.is_buy() {
        (*B::buy_stop.borrow_mut()).get(&order_node.stop_price)
    } else {
        (*B::sell_stop.borrow_mut()).get(&order_node.stop_price)
    };

    let binding: Level;
    let level = match level {
        level => level,
        None => {
            binding = B::add_stop_level(order_book, order_node);
            Some(&binding)
        },
    };

    let level: Level;
    if let Some(level) = level {
        level = level.level;
        B::add_level_volumes(order_book, level, order_node);
        // Link the new order to the orders list of the price level
        level.orders.list.push_back(order_node); 
        level.orders += 1;
        order_node.level = level
    } else {
        level = level.level;
        order_node.level = level
    }
}

pub fn add_trailing_stop_order<'a, C, T, E, H, O, B>(order_book: C, order_node: &OrderNode) 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    let level = if order_node.is_buy() {
        (B::trailing_buy_stop.borrow_mut()).get(&order_node.stop_price)
            .or_else(|| {
                let mut binding: Level = B::add_trailing_stop_level(order_node);
                Some(&binding)
            })// Clones the Arc, not the Level
    } else {
        (B::trailing_sell_stop.borrow_mut()).get(&order_node.stop_price)
            .or_else(|| {
                let mut binding: Level = B::add_trailing_stop_level(order_node);
                Some(&binding)
            }) // Clones the Arc, not the Level
    };

    let node = level;

    let mut level = &node.level;

    // Update the price level volume
    B::add_level_volumes(order_book, *level, order_node);

    // Link the new order to the orders list of the price level
    B::link_order(*level, order_node);

    // Unlink the empty order from the orders list of the price level
    level.orders.list.push_back(order_node);
    level.orders += 1;

    order_node.level = level;
}


pub fn reduce_order<'a, C, T, E, H, O, B>(order_book: C, mut order_node: &'a OrderNode<'a>, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate<'a> 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    let mut update_type = UpdateType::Update;
    let mut level_update: LevelUpdate;

    // remove panicking behavior from code
    let mut level_node = (*order_node.level_node.expect("level node not retrieved from order node").borrow_mut());
    let mut level = level_node.level;
    level.total_volume -= quantity;
    level.hidden_volume -= hidden;
    level.visible_volume -= visible;

    if order_node.order.leaves_quantity == 0 {
        //B::unlink_order(level, order_node)
        level.orders.pop_current(&order_node);
    }

    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_level(order_book, order_node);
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
            tree_node: todo!(),
        },
        top: B::is_top_of_book(order_book, order_node),
    }
}

pub fn delete_order<'a, C, T, E, H, O, B>(order_book: C, order_node: &'a OrderNode<'a>) -> LevelUpdate<'a> 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // remove panicking behavior from code
    let mut level = order_node.level_node.expect("level node not retrieved from order node");
    
    // Update the price level volume
    B::subtract_level_volumes(order_book, level, order_node);

    // Unlink the empty order from the orders list of the price level
    B::unlink_order(order_book, level, order_node);

    let mut update_type = UpdateType::Update;
    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        level = B::delete_level(order_book, order_node).level;
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
            tree_node: todo!(),
        },
        top: B::is_top_of_book(order_book, order_node),
    }
}

pub fn reduce_stop_order<'a, C, T, E, H, O, B>(order_book: C, order_node: &OrderNode, quantity: u64, hidden: u64, visible: u64) 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // Find the price level for the order
    // remove panicking behavior from code
    let mut level_node = order_node.level_node.expect("level node not retrieved from order node");
    let mut level_borrow = level_node.borrow().level;

    // Update the price level volume
    level_borrow.total_volume -= quantity;
    level_borrow.hidden_volume -= hidden;
    level_borrow.visible_volume -= visible;
    // Unlink the empty order from the orders list of the price level
    if order_node.order.leaves_quantity == 0 {
        // Assuming pop_current is a function that removes an order based on Some criteria and returns an Option<order /* OrderNode */>
        level_borrow.orders.pop_current(&order_node);
        // level_borrow.orders -= 1;
    }
    // Delete the empty price level
    if level_borrow.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_stop_level(order_book, order_node);
    };
}

pub fn delete_stop_order<'a, C, T, E, H, O, B>(order_book: C, order_node: &OrderNode) 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{    
    // Update the price level volume
    // Find the price level for the order
    // remove panicking behavior from code
    let mut level_node = order_node.level_node.expect("level node not retrieved from order node");
    let mut level_borrow = level_node.borrow().level;

    level_borrow.total_volume -= order_node.order.leaves_quantity();
    level_borrow.hidden_volume -= order_node.order.hidden_quantity();
    level_borrow.visible_volume -= order_node.order.visible_quantity();

    // Unlink the empty order from the orders list of the price level
    level_borrow.orders.pop_current(&order_node);

    // Delete the empty price level
    if level_borrow.total_volume == 0 {
        B::delete_stop_level(order_book, order_node);
    }
}


pub fn delete_trailing_stop_order<'a, E, C, T, H, O, B>(order_book: C, order_node: &OrderNode) -> Result<(), &'static str> 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
{
    // remove panicking behavior from code
    let mut level_node = order_node.level_node;
    
    // Update the price level volume
    E::subtract_level_volumes(level_node, order_node);

    // Unlink the empty order from the orders list of the price level
    let level = (*level_node.borrow()).level;
    level.orders.pop_current(&order_node); // Assuming each order has a unique identifier

    // Delete the empty price level
    if level.total_volume == 0 {
        // Clear the price level cache in the given order
        B::delete_trailing_stop_level(order_book, order_node);
    };
    Ok(())
}

pub fn delete_stop_level<'a, C>(mut order_book: C, order_node: &OrderNode) 
where 
    C: Deref<Target = OrderBook<'a>>
{
    // remove panicking behavior from code
    let level_node = order_node.level_node.expect("order node level node not retrieved");

    if order_node.is_buy() {
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
        (*stop_level.borrow_mut()).remove(borrowed_level.price);
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
        (*stop_level.borrow_mut()).remove(borrowed_level.price);
    }
}


pub fn activate_stop_orders_level<'a, E, C, T, H, O, B>(order_book: C, mut level: Level, stop_price: u64) -> bool 
where
    C: Deref<Target = OrderBook<'a>>,
    T: Tree<'a>,
    E: Execution<'a>,
    H: Handler,
    O: OrderOps,
    B: OrderBookOperations<'a, C, E, O, H, T>,
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
    while let Some(order_node) = activating_order {
        // Clone next_order to avoid borrow_muting issues
        let next_activating_order = order_node.next_mut();

        match order_node.order_type {
            OrderType::Stop | OrderType::TrailingStop => {
                result |= E::activate_stop_order(order_book, order_node);
            }
            OrderType::StopLimit | OrderType::TrailingStopLimit => {
                result |= E::activate_stop_limit_order(order_book, order_node);
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
