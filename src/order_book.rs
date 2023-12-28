
use std::{collections::BTreeMap, cmp::max, sync::{Mutex, Arc}};
use std::cmp::min;

use crate::{Level, LevelNode, LevelType, LevelUpdate, OrderNodeHandle, UpdateType, level_pool::LevelPool, LevelNodeHandle, order::OrderHandle};

#[derive(Debug)]
pub enum OrderBookError {
    OrderNotFound,
    LevelNotFound,
}

#[derive(Debug, Clone)]
pub struct OrderBookHandle(Arc<Mutex<OrderBook>>);

impl OrderBookHandle {
    pub fn new(order_book: OrderBook) -> Self {
        OrderBookHandle(Arc::new(Mutex::new(order_book)))
    }

    pub fn lock_unwrap(&mut self) -> std::sync::MutexGuard<OrderBook> {
        self.0.lock().expect("Failed to lock OrderBook mutex")
    }
}

#[derive(Debug, Default, Clone)]
pub struct OrderBook
{
    pub best_bid: LevelNodeHandle,
    pub best_ask: LevelNodeHandle,
    pub bids: BTreeMap<u64, LevelNodeHandle>,
    pub asks: BTreeMap<u64, LevelNodeHandle>,

    pub best_buy_stop: LevelNodeHandle,
    pub best_sell_stop: LevelNodeHandle,
    pub buy_stop: BTreeMap<u64, LevelNodeHandle>,
    pub sell_stop: BTreeMap<u64, LevelNodeHandle>,

    pub best_trailing_buy_stop: LevelNodeHandle,
    pub best_trailing_sell_stop: LevelNodeHandle,
    pub trailing_buy_stop: BTreeMap<u64, LevelNodeHandle>,
    pub trailing_sell_stop: BTreeMap<u64, LevelNodeHandle>,

    // Market last and trailing prices
    pub(crate) last_bid_price: u64,
    pub(crate) last_ask_price: u64,
    pub(crate) matching_bid_price: u64,
    pub(crate) matching_ask_price: u64,
    pub trailing_bid_price: u64,
    pub trailing_ask_price: u64,
    pub(crate) level_pool: Arc<Mutex<LevelPool>>,
}

impl OrderBook {
    pub fn new(level_pool: Arc<Mutex<LevelPool>>) -> OrderBook {

        OrderBook {
            // Use Arc::clone to create references to the same LevelNode instance
            best_bid: LevelNodeHandle::default(),
            best_ask: LevelNodeHandle::default(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            best_buy_stop: LevelNodeHandle::default(),
            best_sell_stop: LevelNodeHandle::default(),
            buy_stop: BTreeMap::new(),
            sell_stop: BTreeMap::new(),
            best_trailing_buy_stop: LevelNodeHandle::default(),
            best_trailing_sell_stop: LevelNodeHandle::default(),
            trailing_buy_stop: BTreeMap::new(),
            trailing_sell_stop: BTreeMap::new(),
            last_bid_price: 0,
            last_ask_price: 0,
            matching_bid_price: 0,
            matching_ask_price: 0,
            trailing_bid_price: 0,
            trailing_ask_price: 0,
            level_pool,
        }
    }

    // Function to encapsulate the process of creating a level node
    fn create_level_node(&self, level_type: LevelType, price: u64) -> LevelNodeHandle {
        let mut level_pool = self.level_pool.lock().unwrap();
            level_pool.create(level_type, price)
    }

    fn release_level_node(&self, price: u64) {
        let mut level_pool = self.level_pool.lock().unwrap();
        level_pool.release(price);
    }

    fn create_and_insert_level(&self, order_node_handle: OrderNodeHandle, level_type: LevelType) -> LevelNodeHandle {

        // Create a new price level based on the provided level type
        // Insert the price level into the appropriate collection based on level type
        let node : LevelNodeHandle;
        node = self.create_level_node(level_type.clone(), order_node_handle.clone().lock_unwrap().price);
        match level_type {
            LevelType::Bid => {
                self.clone().bids.insert(node.lock_unwrap().price, node.clone());
            },
            LevelType::Ask => {
                self.clone().asks.insert(node.lock_unwrap().price, node.clone());
            },
            // Handle other level types as needed
        }
        node
    }

    pub fn add_stop_level(&self, order_node_handle: OrderNodeHandle) -> LevelNodeHandle {
        // Determine the level type and price based on the order node
        // Determine the price and create a level node
        let (price, level_node_handle) = if order_node_handle.clone().lock_unwrap().is_buy() {
            let level_node = self.create_level_node(LevelType::Ask, order_node_handle.clone().lock_unwrap().stop_price).clone();
            (order_node_handle.clone().lock_unwrap().stop_price, level_node)
        } else {
            let level_node = self.create_level_node(LevelType::Bid, order_node_handle.clone().lock_unwrap().stop_price).clone();
            (order_node_handle.clone().lock_unwrap().stop_price, level_node)
        };

        if order_node_handle.clone().lock_unwrap().is_buy() {
            self.clone().buy_stop.insert(level_node_handle.clone().lock_unwrap().price, level_node_handle.clone());
            //uninitialized arc pointer
            if *self.best_buy_stop.lock_unwrap() == LevelNode::default() || (level_node_handle.clone().lock_unwrap().price < self.best_buy_stop.lock_unwrap().price) {
                self.clone().best_buy_stop = level_node_handle.clone();
            }
        } else {
            self.clone().sell_stop.insert(level_node_handle.clone().lock_unwrap().price, level_node_handle.clone());
            if *self.best_sell_stop.lock_unwrap() == LevelNode::default() || (level_node_handle.clone().lock_unwrap().price < self.best_sell_stop.lock_unwrap().price) {
                self.clone().best_sell_stop = level_node_handle.clone();
            }
        }
        level_node_handle.clone()
    }
    

    pub fn add_trailing_stop_level(&self, order_node_handle: OrderNodeHandle) -> LevelNodeHandle {

        let (price, level_node_handle) = if order_node_handle.clone().lock_unwrap().is_buy() {
            let level_node = self.create_level_node(LevelType::Ask, order_node_handle.clone().lock_unwrap().stop_price);
            (order_node_handle.clone().lock_unwrap().stop_price, level_node)
        } else {
            let level_node = self.create_level_node(LevelType::Bid, order_node_handle.clone().lock_unwrap().stop_price);
            (order_node_handle.clone().lock_unwrap().stop_price, level_node)
        };
        
        if order_node_handle.clone().lock_unwrap().is_buy() {
            self.clone().trailing_buy_stop.insert(level_node_handle.clone().lock_unwrap().price, level_node_handle.clone());
            // Update the best trailing buy stop order price level
            if *self.best_trailing_buy_stop().lock_unwrap() == LevelNode::default() || (level_node_handle.clone().lock_unwrap().price < self.best_trailing_buy_stop().lock_unwrap().price) {
                self.clone().best_trailing_buy_stop = level_node_handle.clone();
            }
        } else {
            self.clone().trailing_sell_stop.insert(level_node_handle.clone().lock_unwrap().price, level_node_handle.clone());
            // Update the best trailing sell stop order price level
            if *self.best_trailing_sell_stop().lock_unwrap() == LevelNode::default() || (level_node_handle.clone().lock_unwrap().price < self.best_trailing_sell_stop().lock_unwrap().price) {
                self.clone().best_trailing_sell_stop = level_node_handle.clone();
            }
        }
        level_node_handle.clone()
    }

    pub fn delete_trailing_stop_level(&self, order_node_handle: OrderNodeHandle) -> LevelNodeHandle {
        
        let level_node_handle = order_node_handle.clone().lock_unwrap().level_node_handle.clone();
        if order_node_handle.clone().lock_unwrap().is_buy() {
            // Update the best trailing buy stop order price level
            if *level_node_handle.lock_unwrap() == *self.best_trailing_buy_stop().lock_unwrap() {
                self.clone().best_trailing_buy_stop = if *self.best_trailing_buy_stop.lock_unwrap().right.lock_unwrap() != LevelNode::default() {
                    self.best_trailing_buy_stop.lock_unwrap().right.clone()
                } else {
                    self.best_trailing_buy_stop.lock_unwrap().parent.clone()
                }
            }
            // Erase the price level from the trailing buy stop orders collection
            self.clone().trailing_buy_stop.remove(&level_node_handle.lock_unwrap().price);
        } else {
            // Update the best trailing sell stop order price level
            if *level_node_handle.lock_unwrap() == *self.best_trailing_sell_stop().lock_unwrap() {
                self.clone().best_trailing_sell_stop = if *self.best_trailing_sell_stop.lock_unwrap().left.lock_unwrap() != LevelNode::default() {
                    self.best_trailing_sell_stop.lock_unwrap().left.clone()
                } else {
                    self.best_trailing_sell_stop.lock_unwrap().parent.clone()
                }
            }
            // Erase the price level from the trailing sell stop orders collection
            self.clone().trailing_sell_stop.remove(&level_node_handle.lock_unwrap().price);
        }
        // Release the price level
        self.level_pool.lock().unwrap().release(level_node_handle.clone().lock_unwrap().price)
    }

    pub fn delete_trailing_stop_order(&self, order_node_handle: OrderNodeHandle) -> Result<(), &'static str> {

        let mut level = order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level.clone();
        // Update the price level volume
        self.subtract_level_volumes(level.clone(), order_node_handle.clone());

        // Unlink the empty order from the orders list of the price level
        level.order_list.pop_current(&order_node_handle.clone()); // Assuming each order has a unique identifier
        level.orders -= 1; // Adjusting the orders count

        // Delete the empty price level
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_trailing_stop_level(order_node_handle);
        };
        Ok(())
    }

    pub fn get_market_ask_price(&self) -> u64 {
        let best_price = if *self.best_ask.lock_unwrap() != LevelNode::default() {
            self.best_ask.lock_unwrap().price
        } else {
            u64::MAX
        };
        min(best_price, self.matching_ask_price)
    }

    pub fn get_market_bid_price(&self) -> u64 {
        let best_price = if *self.best_bid.lock_unwrap() != LevelNode::default() {
            self.best_bid.lock_unwrap().price
        } else {
            0
        };
        max(best_price, self.matching_bid_price)
    }

    // pub fn get_bid(&mut self, price: u64) -> LevelNodeHandle {
    //     self.bids.get(&price)
    // }

    // fn get_ask(&mut self, price: u64) -> LevelNodeHandle {
    //     self.asks.get(&price)
    // }

    pub fn best_ask(&self) -> LevelNodeHandle {
        self.best_ask.clone()
    }
    pub fn best_bid(&self) -> LevelNodeHandle {
        self.best_bid.clone()
    }

    fn subtract_level_volumes(&self, level: Level, order_node_handle: OrderNodeHandle) {
        level.clone().total_volume -= order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().leaves_quantity;
        level.clone().hidden_volume -= order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().hidden_quantity();
        level.clone().visible_volume -= order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().visible_quantity();
    }

    fn add_level_volumes(&self, level: Level, order_node_handle: OrderNodeHandle) {
        level.clone().total_volume += order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().leaves_quantity;
        level.clone().hidden_volume += order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().hidden_quantity();
        level.clone().visible_volume += order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().visible_quantity();
    }

    pub fn link_order(&self, level: Level, order_node_handle: OrderNodeHandle) {
        level.clone().order_list.pop_current(&order_node_handle.clone()); // push_back for LinkedList
        level.clone().orders += 1;
    }

    pub fn unlink_order(&self, mut level: Level, order_node_handle: OrderNodeHandle) {
        level.order_list.pop_current(&order_node_handle.clone()); 
        level.orders -= 1;
    }
    
    fn is_top_of_book(&self, order_node_handle: OrderNodeHandle) -> bool {
        if let level = order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level.clone() {
            return match order_node_handle.clone().lock_unwrap().is_buy() {
                true => {
                    let best_bid_locked = self.best_bid.lock_unwrap();
                    best_bid_locked.price == level.price
                },
                false => {
                    let best_ask_locked = self.best_ask.lock_unwrap();
                    best_ask_locked.price == level.price
                },
            };
        }
        false
    }

    pub fn add_level(&self, order_node_handle: OrderNodeHandle) -> LevelNodeHandle {

        let level_node_handle: LevelNodeHandle;
        if order_node_handle.clone().lock_unwrap().is_buy() {
            let level_node_handle = self.create_and_insert_level(order_node_handle, LevelType::Bid);
    
            self.clone().bids.insert(level_node_handle.clone().lock_unwrap().price, level_node_handle.clone());
            if *self.best_bid.lock_unwrap() == LevelNode::default() || level_node_handle.clone().lock_unwrap().price > self.best_bid.lock_unwrap().price {
                self.clone().best_bid = level_node_handle.clone()
            }
            level_node_handle.clone()
        } else {
            let level_node_handle = self.create_and_insert_level(order_node_handle, LevelType::Ask);
    
            self.clone().bids.insert(level_node_handle.clone().lock_unwrap().price, level_node_handle.clone());
            if *self.best_ask.lock_unwrap() == LevelNode::default() || level_node_handle.clone().lock_unwrap().price < self.best_ask.lock_unwrap().price {
                self.clone().best_ask = level_node_handle.clone()
            }
            level_node_handle.clone()
        }
    }

    pub fn best_buy_stop(&self) -> LevelNodeHandle {
        self.best_buy_stop.clone()
    }

    fn delete_level(&self, order_node_handle: OrderNodeHandle) -> LevelNodeHandle {
        let level_node_handle = order_node_handle.clone().lock_unwrap().level_node_handle.clone();
        if order_node_handle.clone().lock_unwrap().is_buy() {
            if self.best_bid == level_node_handle {
                // Update the best bid price level
                self.clone().best_bid = if *self.best_bid.lock_unwrap().left.lock_unwrap() != LevelNode::default() {
                    self.best_bid.lock_unwrap().left.clone()
                } else if *self.best_bid.lock_unwrap().parent.lock_unwrap() != LevelNode::default() {
                    self.best_bid.lock_unwrap().parent.clone()
                } else {
                    self.best_bid.lock_unwrap().right.clone()
                };
                self.clone().bids.remove(&level_node_handle.clone().lock_unwrap().price);
            }
            // Erase the price level from the bid collection
        } else {
            if self.best_ask == level_node_handle {
                // Update the best bid price level
                self.clone().best_ask = if *self.best_ask.lock_unwrap().right.lock_unwrap() != LevelNode::default() {
                    self.best_ask.lock_unwrap().right.clone()
                } else if *self.best_ask.lock_unwrap().parent.lock_unwrap() != LevelNode::default() {
                    self.best_ask.lock_unwrap().parent.clone()
                } else {
                    self.best_ask.lock_unwrap().left.clone()
                };
                self.clone().asks.remove(&level_node_handle.clone().lock_unwrap().price);
            }
        }
        LevelNodeHandle::default()
    }

    pub fn get_next_level(& self, level: LevelNodeHandle) -> Option<LevelNodeHandle> {
        if level.lock_unwrap().is_bid() {
            let mut iter = self.bids.range(..level.lock_unwrap().price).rev();
            iter.next().map(|(_price, node)| node.clone())
        } else {
            let mut iter = self.asks.range((level.lock_unwrap().price + 1)..);
            iter.next().map(|(_price, node)| node.clone())
        }
    }

    pub fn add_order(&self, order_node_handle: OrderNodeHandle) -> LevelUpdate {

        let mut update_type = UpdateType::Update;
        // Find the price level for the order
        let mut existing_level_node = if order_node_handle.clone().lock_unwrap().is_buy() {
            self.bids.get(&order_node_handle.clone().clone().lock_unwrap().order_handle.clone().lock_unwrap().price)
        } else {
            self.asks.get(&order_node_handle.clone().clone().lock_unwrap().order_handle.clone().lock_unwrap().price)
        };

        let binding: LevelNodeHandle;
        if let None = existing_level_node {
            binding = self.clone().add_level(order_node_handle.clone());
            existing_level_node = Some(&binding);
            update_type = UpdateType::Add;
        }

        let level: Level  = Default::default();

        if let Some(arc_level_node_handle) = existing_level_node.clone() {
            let mut level = arc_level_node_handle.clone().lock_unwrap().level.clone();
            // Now you have LevelNode and can pass it to the function
            self.add_level_volumes(level.clone(), order_node_handle.clone());
            arc_level_node_handle.lock_unwrap().order_list.push_back(order_node_handle.clone());
            arc_level_node_handle.lock_unwrap().level.orders += 1;
            order_node_handle.clone().lock_unwrap().level_node_handle = arc_level_node_handle.clone();
        }

        LevelUpdate {
            update_type,
            update: Level { 
                level_type: level.clone().level_type, 
                price: level.price, // Similarly for other fields
                total_volume: level.total_volume,
                hidden_volume: level.hidden_volume,
                visible_volume: level.visible_volume,
                order_list: level.order_list,
                orders: level.orders,
            },
            top: self.is_top_of_book(order_node_handle),
        }
    }
    

    pub fn reduce_order(&self, mut order_node_handle: OrderNodeHandle, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate {

        let mut update_type = UpdateType::Update;
        let mut level_update: LevelUpdate;

        let mut level = order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level.clone();
        level.total_volume -= quantity;
        level.hidden_volume -= hidden;
        level.visible_volume -= visible;

        if order_node_handle.clone().lock_unwrap().leaves_quantity == 0 {
            //self.unlink_order(level, order_node_handle)
            level.order_list.pop_current(&order_node_handle.clone());
            level.orders -= 1
        }

        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_level(order_node_handle.clone());
            order_node_handle.clone().lock_unwrap().level_node_handle = self.delete_level(order_node_handle.clone());
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
                order_list: level.order_list,
                orders: level.orders,
            },
            top: self.is_top_of_book(order_node_handle),
        }
    }

    pub fn delete_order(&self, order_node_handle: OrderNodeHandle) -> LevelUpdate {

        let mut level = order_node_handle.clone().lock_unwrap().level_node_handle.clone().lock_unwrap().level.clone();
        
        // Update the price level volume
        self.subtract_level_volumes(level.clone(), order_node_handle.clone());

        // Unlink the empty order from the orders list of the price level
        self.unlink_order(level.clone(), order_node_handle.clone());

        let mut update_type = UpdateType::Update;
        if level.clone().total_volume == 0 {
            // Clear the price level cache in the given order
            level = self.delete_level(order_node_handle.clone()).lock_unwrap().level.clone();
            update_type = UpdateType::Delete;
        }
        LevelUpdate {
            update_type,
            update: Level { 
                level_type: level.clone().level_type, 
                price: level.clone().price, // Similarly for other fields
                total_volume: level.clone().total_volume,
                hidden_volume: level.clone().hidden_volume,
                visible_volume: level.clone().visible_volume,
                order_list: level.clone().order_list,
                orders: level.clone().orders,
            },
            top: self.is_top_of_book(order_node_handle),
        }
    }
    

    fn delete_stop_level(&self, order_node_handle: OrderNodeHandle) {
        let level_node_handle = order_node_handle.clone().lock_unwrap().level_node_handle.clone();

        if order_node_handle.clone().lock_unwrap().is_buy() {
            // Update the best buy stop order price level
            if self.best_buy_stop == level_node_handle {
                self.clone().best_buy_stop = if *self.best_buy_stop.lock_unwrap().right.lock_unwrap() != LevelNode::default() {
                    self.best_buy_stop.lock_unwrap().right.clone()
                } else {
                    self.best_buy_stop.lock_unwrap().parent.clone()
                }
            }
            // Erase the price level from the buy stop orders collection
            self.clone().buy_stop.remove(&level_node_handle.lock_unwrap().price);
        } else {
            if self.best_sell_stop == level_node_handle {
                // Update the best sell stop order price level
                self.clone().best_sell_stop = if *self.best_sell_stop.lock_unwrap().right.lock_unwrap() != LevelNode::default() {
                    self.best_sell_stop.lock_unwrap().right.clone()
                } else {
                    self.best_sell_stop.lock_unwrap().parent.clone()
                }
            }
            // Erase the price level from the sell stop orders collection
            self.clone().sell_stop.remove(&level_node_handle.lock_unwrap().price);
        }

        // Release the price level
        // Assuming you have a method in your Rust implementation similar to C++'s Release
        self.level_pool.lock().unwrap().release(level_node_handle.lock_unwrap().price);
    }

    pub fn add_stop_order(&self, order_node_handle: OrderNodeHandle) {
        // Find the price level for the order
        let level_node_handle = if order_node_handle.clone().lock_unwrap().is_buy() {
            self.buy_stop.get(&order_node_handle.clone().clone().lock_unwrap().stop_price)
        } else {
            self.sell_stop.get(&order_node_handle.clone().clone().lock_unwrap().stop_price)
        };

        let binding: LevelNodeHandle;
        let level_node_handle = match level_node_handle {
            level_node_handle => level_node_handle,
            None => {
                binding = self.clone().add_stop_level(order_node_handle.clone());
                Some(&binding)
            },
        };

        let level: Level;
        if let Some(level_node_handle) = level_node_handle {
            level = level_node_handle.lock_unwrap().level.clone();
            self.add_level_volumes(level.clone(), order_node_handle.clone());
            // Link the new order to the orders list of the price level
            level.clone().order_list.list.push_back(order_node_handle.clone()); 
            level.clone().orders += 1;
            order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level = level.clone();
        } else {
            level = level_node_handle.unwrap().lock_unwrap().level.clone();
            order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level = level.clone();
        }
    }

  pub fn reduce_stop_order(&self, order_node_handle: OrderNodeHandle, quantity: u64, hidden: u64, visible: u64) {
        
        // Find the price level for the order
        let mut level = order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level.clone();

        // Update the price level volume
        level.total_volume -= quantity;
        level.hidden_volume -= hidden;
        level.visible_volume -= visible;
        // Unlink the empty order from the orders list of the price level
        if order_node_handle.clone().lock_unwrap().leaves_quantity == 0 {
            // Assuming pop_current is a function that removes an order based on Some criteria and returns an Option<order /* OrderNodeHandle */>
            level.order_list.pop_current(&order_node_handle.clone());
            level.orders -= 1;
        }
        // Delete the empty price level
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_stop_level(order_node_handle);
        };
    }

    pub fn delete_stop_order(&self, order_node_handle: OrderNodeHandle) {
        
        // Update the price level volume
        let mut level = order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level.clone();
        level.total_volume -= order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().leaves_quantity;
        level.hidden_volume -= order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().hidden_quantity();
        level.visible_volume -= order_node_handle.clone().lock_unwrap().order_handle.clone().lock_unwrap().visible_quantity();

        // Unlink the empty order from the orders list of the price level
        let _ = level.order_list.pop_current(&order_node_handle.clone()).ok_or("Failed to remove order from order list");
        level.orders -= 1;

        // Delete the empty price level
        if level.total_volume == 0 {
            self.delete_stop_level(order_node_handle);
        }
    }

    pub fn reduce_trailing_stop_order(&self, mut order_node_handle: OrderNodeHandle, quantity: u64, hidden: u64, visible: u64) {
        // Assuming we have a way to get a mutable reference to an order and its level.
        // Update the price level volume
        let mut level = order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level.clone();
        // Update the price level volume
        self.subtract_level_volumes(level.clone(), order_node_handle.clone());
        // Unlink the empty order from the orders list of the price level
        if order_node_handle.clone().lock_unwrap().leaves_quantity == 0 {
            self.unlink_order(level.clone(), order_node_handle.clone());
        }
        // Delete the empty price level
        if level.clone().total_volume == 0 {
            order_node_handle.clone().lock_unwrap().level_node_handle = self.delete_trailing_stop_level(order_node_handle.clone())
        }
    }

    // Method to get the best trailing buy stop level
    pub fn best_trailing_buy_stop(&self) -> LevelNodeHandle {
        self.best_trailing_buy_stop.clone()
    }

    // Method to get the best trailing sell stop level
    pub fn best_trailing_sell_stop(&self) -> LevelNodeHandle {
        self.best_trailing_sell_stop.clone()
    }

    // Method to get the best sell stop level
    pub fn best_sell_stop(&self) -> LevelNodeHandle {
        self.best_sell_stop.clone()
    }

    pub fn get_market_trailing_stop_price_ask(&self) -> u64 {
        let last_price = self.last_ask_price;
        let best_price = if *self.best_ask.lock_unwrap() != LevelNode::default() {
            self.best_ask.lock_unwrap().price
        } else {
            u64::MAX
        };
        std::cmp::max(last_price, best_price)
    }

    pub fn get_market_trailing_stop_price_bid(&self) -> u64 {
        let last_price = self.last_bid_price;
        let best_price = if *self.best_bid.lock_unwrap() != LevelNode::default() {
            self.best_bid.lock_unwrap().price
        } else {
            0
        };
        std::cmp::min(last_price, best_price)
    }

    // Method to get the trailing buy stop level
    // pub fn get_trailing_buy_stop_level(&mut self, price: u64) -> LevelNodeHandle {
    //     self.trailing_buy_stop.get(&price)
    // }

    // // Method to get the trailing sell stop level
    // pub fn get_trailing_sell_stop_level(&mut self, price: u64) -> LevelNodeHandle {
    //     self.trailing_sell_stop.get(&price)
    // }

  pub fn add_trailing_stop_order(& self, order_node_handle: OrderNodeHandle) {

        let level_node_handle = if order_node_handle.clone().lock_unwrap().is_buy() {
            let mut binding: LevelNodeHandle = Default::default();
            self.trailing_buy_stop.get(&order_node_handle.clone().clone().lock_unwrap().stop_price)
                .or_else(|| {
                    binding = self.clone().add_trailing_stop_level(order_node_handle.clone());
                    Some(&binding)
                })
                .cloned() // Clones the Arc, not the LevelNode
        } else {
            let mut binding: LevelNodeHandle = Default::default();
            self.trailing_sell_stop.get(&order_node_handle.clone().clone().lock_unwrap().stop_price)
                .or_else(|| {
                    binding = self.clone().add_trailing_stop_level(order_node_handle.clone());
                    Some(&binding)
                })
                .cloned() // Clones the Arc, not the LevelNode
        };

        let node = level_node_handle.unwrap();

        let mut level = &node.lock_unwrap().level;

        // Update the price level volume
        self.add_level_volumes(level.clone(), order_node_handle.clone());

        // Link the new order to the orders list of the price level
        self.link_order(level.clone(), order_node_handle.clone());

        // Unlink the empty order from the orders list of the price level
        level.clone().order_list.list.push_back(order_node_handle.clone());
        level.clone().orders += 1;

        order_node_handle.clone().lock_unwrap().level_node_handle.lock_unwrap().level = level.clone();
    }

    pub fn get_next_trailing_stop_level(&self, level_node_handle: LevelNodeHandle) -> LevelNodeHandle {
        let level = level_node_handle.lock_unwrap();
        if level.is_bid() {
            // Find the next level in reverse order in _trailing_sell_stop
            let var= self.trailing_sell_stop
                .range(..level.price).rev() // Iterate in reverse up to the current price
                .next()
                .unwrap()
                .1;
            var.clone()  // Return the node if found
        } else {
            // Find the next level in normal order in _trailing_buy_stop
            let var = self.trailing_buy_stop
                .range((level.price + 1)..) // Iterate starting from just above the current price
                .next()
                .unwrap()                     // Get the next element
                .1;
            var.clone()
        }
    }

  pub fn calculate_trailing_stop_price(&self, order_handle: OrderHandle) -> u64 {
        // Get the current market price
        let market_price = if order_handle.clone().lock_unwrap().is_buy() {
            self.get_market_trailing_stop_price_ask()
        } else {
            self.get_market_trailing_stop_price_bid()
        };
        let mut trailing_distance = order_handle.clone().lock_unwrap().trailing_distance as i64;
        let mut trailing_step = order_handle.clone().lock_unwrap().trailing_step as i64;

        // Convert percentage trailing values into absolute ones
        if trailing_distance < 0 {
            trailing_distance = -trailing_distance * market_price as i64 / 10000;
            trailing_step = -trailing_step * market_price as i64 / 10000;
        }

        let old_price = order_handle.clone().lock_unwrap().stop_price;

        if order_handle.clone().lock_unwrap().is_buy() {
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

    // pub fn update_level(&mut self, order_book: OrderBook, update: LevelUpdate) {
        
    //     match update.update_type {
    //         UpdateType::Add => market_handler.on_add_level(order_book, &update.update, update.top),
    //         UpdateType::Update => market_handler.on_update_level(order_book, &update.update, update.top),
    //         UpdateType::Delete => market_handler.on_delete_level(order_book, &update.update, update.top),
    //         _ => return,
    //     };
    //     market_handler.on_update_order_book(order_book, update.top)
    // }
    
    pub fn reset_matching_price(&self) {
        self.clone().matching_bid_price = 0;
        self.clone().matching_ask_price = u64::MAX;
    }

    pub fn on_trailing_stop(&self, order_handle: OrderHandle) {
        // Here you would implement the specific logic for handling a trailing stop order
        // For example:
        if order_handle.clone().lock_unwrap().is_buy() {
            // Handle trailing stop for buy order
            // Update order book, prices, or other states as required
        } else {
            // Handle trailing stop for sell order
            // Update order book, prices, or other states as required
        }

        // Other logic as needed for trailing stops...
    }

    pub fn update_last_price(&self, order_handle: OrderHandle, price: u64) {
        if order_handle.clone().lock_unwrap().is_buy() {
            self.clone().last_bid_price = price;
        } else {
            self.clone().last_ask_price = price;
        }
    }

    pub fn update_matching_price(&self, order_handle: OrderHandle, price: u64) {
        if order_handle.clone().lock_unwrap().is_buy() {
            self.clone().matching_bid_price = price;
        } else {
            self.clone().matching_ask_price = price;
        }
    }
}
