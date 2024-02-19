

use std::{cell::RefCell, collections::VecDeque, rc::Rc};
use crate::{levels::{indexing::{LevelNode, TreeOps, TreeRemoval}, level::{self, Level, LevelOps, LevelType, LevelUpdate, PopCurrent, UpdateType}}, market_executors::executor::Execution, market_handler::Handler, orders::{command::Command, order::{ErrorCode, Order, OrderType}, orders::OrderOps}};

#[derive(Debug)]
pub enum OrderBookError {
    OrderNotFound,
    LevelNotFound,
}

#[derive(Default)]
pub struct OrderBook {
    pub best_bid: Option<Rc<RefCell<LevelNode>>>,
    pub best_ask: Option<Rc<RefCell<LevelNode>>>,
    pub bids: Option<Rc<RefCell<LevelNode>>>,
    pub asks: Option<Rc<RefCell<LevelNode>>>,

    pub best_buy_stop: Option<Rc<RefCell<LevelNode>>>,
    pub best_sell_stop: Option<Rc<RefCell<LevelNode>>>,
    pub buy_stop: Option<Rc<RefCell<LevelNode>>>,
    pub sell_stop: Option<Rc<RefCell<LevelNode>>>,

    pub(crate) last_bid_price: u64,
    pub(crate) last_ask_price: u64,
    pub(crate) matching_bid_price: u64,
    pub(crate) matching_ask_price: u64,

    pub best_trailing_buy_stop: Option<Rc<RefCell<LevelNode>>>,
    pub best_trailing_sell_stop: Option<Rc<RefCell<LevelNode>>>,
    pub trailing_buy_stop: Option<Rc<RefCell<LevelNode>>>,
    pub trailing_sell_stop: Option<Rc<RefCell<LevelNode>>>,
    pub trailing_bid_price: u64,
    pub trailing_ask_price: u64,
}

macro_rules! get_next_level {
    ($field:expr, $level_node:expr, $lower:ident, $higher:ident) => {{
        $field
            .ok_or(ErrorCode::DefaultError)
            .and_then(|node| {
                if $level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.is_bid() {
                    node.$lower($level_node)
                } else {
                    node.$higher($level_node)
                }
            })
    }};
}

macro_rules! access_price {
    ($node:expr, $default:expr, $last_price:expr) => {
        {
            let best_price = $node.map_or_else(
                || $default, // Default value if `node` is None.
                |node| node.try_borrow().map_or_else(
                    |_| $default, // Return default in case of a borrow error.
                    |borrowed_node| borrowed_node.level.price, // Return the price on successful borrow.
                ),
            );
            std::cmp::min($last_price, best_price)
        }
    };
}

macro_rules! with_level {
    ($order:expr, $action:expr) => {
        $order.level.as_ref().ok_or(ErrorCode::DefaultError)
            .and_then(|level| {
                level.try_borrow_mut().map_err(|_| ErrorCode::DefaultError)
                    .and_then(|mut level| $action(&mut level))
            })
    };
}

impl OrderBook {
    pub fn new() -> OrderBook {
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
            command: todo!(),
        }
    }

    // Method to get the best trailing buy stop level
    pub fn best_trailing_buy_stop(&self) -> Option<Rc<RefCell<LevelNode>>> {
        self.best_trailing_buy_stop.clone()
    }

    // Method to get the best trailing sell stop level
    pub fn best_trailing_sell_stop(&self) -> Option<Rc<RefCell<LevelNode>>> {
        self.best_trailing_sell_stop.clone()
    }

    pub fn get_trailing_buy_stop_level(&mut self, price: u64) -> Option<Rc<RefCell<LevelNode>>> {
        self.trailing_buy_stop
            .and_then(|mut node| node.find_node_by_price(price))
    }

    pub fn get_trailing_sell_stop_level(&mut self, price: u64) -> Option<Rc<RefCell<LevelNode>>> {
        self.trailing_sell_stop
            .and_then(|mut node| node.find_node_by_price(price))
    }

    #[cfg(feature = "macro")]
    pub fn get_next_trailing_stop_level(&mut self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        get_next_level!(self.trailing_sell_stop, level_node, get_next_lower_level, get_next_higher_level)
    }
    #[cfg(feature = "macro")]
    pub fn get_next_level_node(&self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        get_next_level!(self.bids, level_node, get_next_lower_level, get_next_higher_level)
    }

    // Method to get the next trailing stop level based on whether it's a bid or ask

    pub fn get_next_trailing_stop_level(&mut self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        if level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.is_bid() {
            self.trailing_sell_stop
                .clone()
                .ok_or(ErrorCode::DefaultError)
                .and_then(|node| node.get_next_lower_level(level_node))
        } else {
            self.trailing_buy_stop
                .clone()
                .ok_or(ErrorCode::DefaultError)
                .and_then(|node| node.get_next_higher_level(level_node))
        }
    }

    // Method to get the next level node based on whether it's a bid or ask
    pub fn get_next_level_node(&self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        if level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.is_bid() {
            self.bids
                .clone()
                .ok_or(ErrorCode::DefaultError)
                .and_then(|node| node.get_next_lower_level(level_node))
        } else {
            self.asks
                .clone()
                .ok_or(ErrorCode::DefaultError)
                .and_then(|node| node.get_next_higher_level(level_node))
        }
    }

    pub fn delete_trailing_stop_level(&mut self, order: &Order) -> Result<(), ErrorCode> {

        let (collection, opposite_collection, is_buy) = if order.is_buy() {
            (&mut self.best_trailing_buy_stop, &mut self.trailing_sell_stop, true)
        } else {
            (&mut self.best_trailing_sell_stop, &mut self.trailing_buy_stop, false)
        };

        let level_node = order
            .level_node
            .as_ref()
            .ok_or(ErrorCode::DefaultError)?;
        let best_stop = collection
            .as_ref()
            .ok_or(ErrorCode::DefaultError)?;
        let mut borrow_stop = best_stop
            .try_borrow_mut()
            .map_err(|_| ErrorCode::DefaultError)?;
        
        if *best_stop == *level_node {
            let price = borrow_stop.level.price;
            if is_buy {
                *collection = borrow_stop
                    .right
                    .take()
                    .or_else(|| {
                        borrow_stop.parent.as_ref().and_then(|weak_parent| 
                            weak_parent.upgrade()
                        ).map(|upgraded_parent| 
                            upgraded_parent.clone() // Clone after successful upgrade
                        )
                    });
            } else {
                *collection = borrow_stop
                    .left
                    .take()
                    .or_else(|| {
                        borrow_stop.parent.as_ref().and_then(|weak_parent| 
                            weak_parent.upgrade()
                        ).map(|upgraded_parent| 
                            upgraded_parent.clone() // Clone after successful upgrade
                        )
                    });
            }
            opposite_collection
                .expect("should get opposite collection")
                .remove(price)
                .map_err(|_| ErrorCode::DefaultError)?;
        }
        Ok(())
    }

    pub fn add_trailing_stop_level(&mut self, order: &Order) -> Result<Rc<RefCell<LevelNode>>, ErrorCode> {
        let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(
            if order.is_buy() { LevelType::Ask } else { LevelType::Bid },
            order.stop_price,
        ))));

        let (collection, is_buy) = if order.is_buy() {
            (&mut self.trailing_buy_stop, true)
        } else {
            (&mut self.trailing_sell_stop, false)
        };

        collection.insert(level_node.clone());
        
        let update_condition = if is_buy {
            self.best_trailing_buy_stop.as_ref().map_or(true, |best| level_node.try_borrow().map(|node| node.level.price < best.try_borrow().map_or(u64::MAX, |best| best.level.price)).unwrap_or(false))
        } else {
            self.best_trailing_sell_stop.as_ref().map_or(true, |best| level_node.try_borrow().map(|node| node.level.price < best.try_borrow().map_or(u64::MAX, |best| best.level.price)).unwrap_or(false))
        };

        if update_condition {
            if is_buy {
                self.best_trailing_buy_stop = Some(level_node.clone());
            } else {
                self.best_trailing_sell_stop = Some(level_node.clone());
            }
        }

        Ok(level_node)
    }

    pub fn best_buy_stop(&self) -> Option<Rc<RefCell<LevelNode>>> 
    {
        self.best_buy_stop.clone()
    }

    // Method to get the best sell stop level
    pub fn best_sell_stop(&self) -> Option<Rc<RefCell<LevelNode>>> 
    {
        self.best_sell_stop.clone()
    }

    pub fn add_stop_level(&mut self, order: &Order) -> Result<Rc<RefCell<LevelNode>>, ErrorCode> {
        let level_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(
            if order.is_buy() { LevelType::Ask } else { LevelType::Bid },
            order.stop_price,
        ))));

        let comparison_result = if order.is_buy() {
            if let Some(best_stop) = &self.best_buy_stop {
                level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.price < best_stop.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.price
            } else {
                true
            }
        } else {
            if let Some(best_stop) = &self.best_sell_stop {
                level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.price < best_stop.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.price
            } else {
                true
            }
        };

        if comparison_result {
            if order.is_buy() {
                self.best_buy_stop = Some(level_node.clone());
            } else {
                self.best_sell_stop = Some(level_node.clone());
            }
        }

        if order.is_buy() {
            self.buy_stop.insert(level_node.clone());
        } else {
            self.sell_stop.insert(level_node.clone());
        }

        Ok(level_node)
    }

    pub fn create_and_insert_level(&mut self, price: u64, level_type: LevelType) -> Result<Rc<RefCell<LevelNode>>, ErrorCode> {
        let new_node = Rc::new(RefCell::new(LevelNode::from(Level::with_price(level_type, price))));

        match level_type {
            LevelType::Bid => {
                self.bids.insert(new_node.clone());
            },
            LevelType::Ask => {
                self.asks.insert(new_node.clone());
            },
        }
        Ok(new_node)
    }

    pub fn add_stop_order(&mut self, order: &Order) -> Result<(), ErrorCode> {

        let level_node = if order.is_buy() {
            self.buy_stop
                .ok_or(ErrorCode::DefaultError)?
                .find_node_by_price(order.stop_price)
        } else {
            self.sell_stop
                .ok_or(ErrorCode::DefaultError)?
                .find_node_by_price(order.stop_price)
        };

        {
            level_node
                .expect("should have gotten node")
                .try_borrow_mut()
                .and_then(|mut level_node| Ok({
                    level_node.level.add_volumes(order); // Assuming this returns Result<(), ErrorCode>
                    level_node.level.orders.push_back(*order);
                }));
        }

        order.level_node = level_node.clone();

        Ok(())
    }

    pub fn add_trailing_stop_order(&mut self, order: &Order) -> Result<(), ErrorCode> {

        let level_node = if order.is_buy() {
            self.get_trailing_buy_stop_level(order.stop_price)
        } else {
            self.get_trailing_sell_stop_level(order.stop_price)
        };

        // Proceed with the level node if found.
        level_node.and_then(|holder| {
            // Ensure we have a level node to work with.
            // Attempt to borrow the holder mutably.
            holder.try_borrow_mut().ok().and_then(|mut node| Some({
                // If successful, apply changes to the level.
                let level = &mut node.level;
                level.add_volumes(order); 
                level.link_order(order);  
                level.orders.push_back(*order);
            }))
            }// Handle the case where no level node is found.
        ).ok_or(ErrorCode::DefaultError)
    }
    

    // Function to delete a level from the order book based on a given order.
    // It updates the best bid or ask pointers and removes the level from the bids or asks collection.
    pub fn delete_level(&mut self, order: &Order) -> Result<(), ErrorCode> {
        // Retrieve the level node from the order, returning an error if it's not set.
        let level_node = order.level_node
            .ok_or(ErrorCode::DefaultError)?;

        // Check if the order is a buy order.
        if order.is_buy() {
            // Retrieve the current best bid, returning an error if it's not set.
            let best_bid = self.best_bid
                .ok_or(ErrorCode::DefaultError)?;
            // Check if the best bid is the same as the level node associated with the order.
            if best_bid == level_node {
                // Try to borrow the best bid mutably, returning an error if it fails.
                let mut borrowed_best = best_bid
                    .try_borrow_mut()
                    .map_err(|_| ErrorCode::DefaultError)?;
                // Update the best bid reference, prioritizing the left child, then the parent, and finally the right child.
                self.best_bid = borrowed_best
                    .left
                    .clone()
                    .or_else(|| borrowed_best
                        // Upgrade the parent from a Weak to an Rc, if possible, and wrap it in a Holder.
                        .parent
                        .map(|parent| 
                            parent.upgrade()
                            .expect("parent")))
                            .clone()
                            .or(borrowed_best
                                .right
                                .clone());
                // Remove the level based on its price from the bids collection, returning an error if it fails.
                self.bids
                    .ok_or(ErrorCode::DefaultError)?
                    .remove(borrowed_best.level.price)?;
            }
        } else {
            // The logic for sell orders mirrors that of buy orders, with the difference being it operates on the best ask.
            let best_ask = self.best_ask.ok_or(ErrorCode::DefaultError)?;
            if best_ask == level_node {
                let mut borrowed_best = best_ask
                    .try_borrow_mut()
                    .map_err(|_| ErrorCode::DefaultError)?;
                self.best_ask = borrowed_best
                    .left
                    .clone()
                    .or_else(|| borrowed_best
                        .parent
                        .map(|parent| 
                            parent.upgrade()
                            .expect("parent")))
                            .clone()
                            .or(borrowed_best
                                .right
                                .clone());
                self.asks
                    .ok_or(ErrorCode::DefaultError)?
                    .remove(borrowed_best.level.price)?;
            }
        }

        // Return Ok to indicate the operation completed successfully.
        Ok(())
    }

    pub fn add_level(&mut self, order: &Order) -> Result<Rc<RefCell<LevelNode>>, ErrorCode> {
        let level_type = if order.is_buy() { LevelType::Bid } else { LevelType::Ask };
        let level_node = self.create_and_insert_level(order.price, level_type);
        let level = level_node
            .expect("level node insertion")
            .try_borrow()
            .map_err(|_| ErrorCode::DefaultError)?
            .level;

        if order.is_buy() {
            if self.best_bid.map_or(true, |bid| level.price > bid.try_borrow().ok().map_or(u64::MAX, |b| b.level.price)) {
                self.best_bid = level_node.map(|node| node.clone()).ok();
            }
        } else {
            if self.best_ask.map_or(true, |ask| level.price < ask.try_borrow().ok().map_or(0, |a| a.level.price)) {
                self.best_ask = level_node.map(|node| node.clone()).ok();
            }
        }
        level_node
    }

    pub fn best_ask(& self) -> Option<Rc<RefCell<LevelNode>>>                              
    {
        self.best_ask.clone()
    }

    pub fn best_bid(&self) -> Option<Rc<RefCell<LevelNode>>>                                   
    {
        self.best_bid.clone()
    } 

    pub fn get_bid(&mut self, price: u64) -> Result<Rc<RefCell<LevelNode>>, ErrorCode> {
        self.bids
            .as_ref()
            .ok_or(ErrorCode::DefaultError)?
            .find_node_by_price(price)
            .ok_or(ErrorCode::DefaultError)
    }

    pub fn get_ask(&mut self, price: u64) -> Result<Rc<RefCell<LevelNode>>, ErrorCode> {
        self.asks
            .as_ref()
            .ok_or(ErrorCode::DefaultError)?
            .find_node_by_price(price)
            .ok_or(ErrorCode::DefaultError)
    }

    pub fn get_market_ask_price(&self) -> u64 {
        access_price!(self.best_ask, u64::MAX, self.last_ask_price)
    }
    
    pub fn get_market_bid_price(&self) -> u64 {
        access_price!(self.best_bid, 0, self.last_bid_price)
    }
    
    pub fn get_market_trailing_stop_price_ask(&mut self) -> u64 {
        access_price!(self.best_ask, u64::MAX, self.last_ask_price)
    }
    
    pub fn get_market_trailing_stop_price_bid(&mut self) -> u64 {
        access_price!(self.best_bid, 0, self.last_bid_price)
    }
    
    #[cfg(feature = "no macro")]
    pub fn get_market_ask_price(&self) -> u64                                         
    {
        let last_price = self.last_ask_price;
        let best_price = self.best_ask.map_or_else(
        || u64::MAX, // Default to MAX if `best_ask` is None.
        |ask_node| ask_node.try_borrow().map_or_else(
            |_| u64::MAX, // In case of a borrow error, return MAX.
            |node| node.level.price, // On successful borrow, return the price.
            ),
        );
        std::cmp::min(last_price, best_price)
    }

    #[cfg(feature = "no macro")]
    pub fn get_market_bid_price(&self) -> u64                                          
    {
        let last_price = self.last_bid_price;
        let best_price = self.best_bid.map_or_else(
        || 0, // Default to 0 if `best_bid` is None.
        |bid_node| bid_node.try_borrow().map_or_else(
            |_| 0, // In case of a borrow error, return 0.
            |node| node.level.price, // On successful borrow, return the price.
            ),
        );
        std::cmp::max(last_price, best_price)
    }

    #[cfg(feature = "no macro")]
    pub fn get_market_trailing_stop_price_ask(&mut self) -> u64                                      
    { 
        let last_price = self.last_ask_price;
        let best_price = self.best_ask.map_or(u64::MAX, |ask_node| {
            ask_node.try_borrow().map_or_else(
                |_| u64::MAX, // In case of a borrow error, return a default price.
                |node| node.level.price, // On successful borrow, return the price.
            )
        });
        std::cmp::max(last_price, best_price)
    }

    #[cfg(feature = "no macro")]
    pub fn get_market_trailing_stop_price_bid(&mut self) -> u64                                           
    {
        let last_price = self.last_bid_price;
        let best_price = self.best_bid.map_or_else(
            || 0, // Default to 0 if `best_bid` is None.
            |bid_node| bid_node.try_borrow().map_or_else(
                |_| 0, // In case of a borrow error, return 0.
                |node| node.level.price, // On successful borrow, return the price.
            ),
        );
        std::cmp::min(last_price, best_price)
    }

    pub fn is_top_of_book(&mut self, order: &Order) -> bool                                          
    {
        order.level_node.as_ref().map_or(false, |level_node| {
            return match order.is_buy() {
                true => {
                    self.best_bid
                        .as_ref()
                        .and_then(|bid| bid.try_borrow().ok())
                        .map(|bid_ref| bid_ref.level.price == level_node.try_borrow().ok().map_or(0, |ln| ln.level.price))
                        .unwrap_or(false)
                },
                false => {
                    self.best_ask
                    .as_ref()
                    .and_then(|ask| ask.try_borrow().ok())
                    .map(|ask_ref| ask_ref.level.price == level_node.try_borrow().ok().map_or(0, |ln| ln.level.price))
                    .unwrap_or(false)
                },
            };
        })
    }

    pub fn on_trailing_stop(&mut self, order: &Order)                                    
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

    pub fn reset_matching_price(&mut self)                                        
    {
        self.matching_bid_price = 0;
        self.matching_ask_price = u64::MAX;
    }

    pub fn update_last_price(&mut self, order: &Order, price: u64)                                          
    {
        if order.is_buy() {
            self.last_bid_price = price;
        } else {
            self.last_ask_price = price;
        }
    }

    pub fn update_matching_price(&mut self, order: &Order, price: u64)                              
    {
        if order.is_buy() {
            self.matching_bid_price = price;
        } else {
            self.matching_ask_price = price;
        }
    }

    pub fn calculate_trailing_stop_price(&mut self, order: &Order) -> Result<u64, ErrorCode> {
        // Get the current market price
        let market_price = if order.is_buy() {
            self.get_market_trailing_stop_price_ask()
        } else {
            self.get_market_trailing_stop_price_bid()
        };

        let trailing_distance = order.trailing_distance as i64; // Assuming trailing_distance is i64
        let trailing_step = order.trailing_step as i64; // Assuming trailing_step is i64

        // Check for valid percentage values and calculate absolute ones
        if trailing_distance < 0 || trailing_step < 0 {
            return Err(ErrorCode::DefaultError);
        }

        let trailing_distance = if trailing_distance < 0 {
            // Assuming you meant to check if it's a percentage value to convert it
            trailing_distance - trailing_distance * market_price as i64 / 10000
        } else {
            trailing_distance
        };

        let trailing_step = if trailing_step < 0 {
            // Similarly for trailing_step
            trailing_step - trailing_step * market_price as i64 / 10000
        } else {
            trailing_step
        };

        let old_price = order.stop_price; // Assuming stop_price is u64, convert for calculation

        let new_price = if order.is_buy() {
            market_price.checked_add(trailing_distance as u64)
                .ok_or(ErrorCode::DefaultError)?
        } else {
            market_price.checked_sub(trailing_distance as u64)
                .ok_or(ErrorCode::DefaultError)?
        };

        let price_difference = i64::abs(old_price as i64 - new_price as i64);

        if (order.is_buy() && new_price < old_price && price_difference >= trailing_step) ||
           (!order.is_buy() && new_price > old_price && price_difference >= trailing_step) {
            Ok(new_price)
        } else {
            Ok(old_price as u64) // Converting back if no changes are needed
        }
    }

    pub fn recalculate_trailing_stop_price<E>(&mut self, level_node: Option<Rc<RefCell<LevelNode>>>) -> Result<(), ErrorCode>
    where
        E: Execution + Handler + OrderOps,
    {
        let level_node = level_node.ok_or(ErrorCode::DefaultError)?;
        let level_type = level_node.try_borrow().map_err(|_| ErrorCode::DefaultError)?.level.level_type;

        let new_trailing_price = match level_type {
            LevelType::Ask => {
                let old_trailing_price = self.trailing_ask_price;
                let new_price = self.get_market_trailing_stop_price_ask();
                if new_price >= old_trailing_price {
                    return Ok(());
                }
                self.trailing_ask_price = new_price;
                new_price
            },
            LevelType::Bid => {
                let old_trailing_price = self.trailing_bid_price;
                let new_price = self.get_market_trailing_stop_price_bid();
                if new_price <= old_trailing_price {
                    return Ok(());
                }
                self.trailing_bid_price = new_price;
                new_price
            },
        };

        // Assuming `best_trailing_buy_stop` and `best_trailing_sell_stop` are Option<Rc<RefCell<LevelNode>>>
        let mut current = match level_type {
            LevelType::Ask => self.best_trailing_buy_stop.clone(),
            LevelType::Bid => self.best_trailing_sell_stop.clone(),
        };

        let mut previous: Option<Rc<RefCell<LevelNode>>> = None;

        while let Some(current_level) = current {
            let mut recalculated = false;
            // Assuming `orders` field exists and it's a collection that can be iterated over
            let orders = current_level.try_borrow_mut().map_err(|_| ErrorCode::DefaultError)?.level.orders.clone(); // Clone to avoid borrowing issues
            
            for order in orders.iter() { // Adjusted for a more idiomatic iteration
                let old_stop_price = order.stop_price;
                let new_stop_price = self.calculate_trailing_stop_price(order)?; // Assuming this method is infallible or adjusted to return Result
                
                if new_stop_price != old_stop_price {
                    self.delete_trailing_stop_order(&order)?;
                    match order.order_type {
                        OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                            // Assuming `on_update_order` and `add_trailing_stop_order` are adjusted to handle Results
                            E::on_update_order(&order);
                            self.add_trailing_stop_order(&order)?;
                        },
                        _ => return Err(ErrorCode::DefaultError),
                    }
                    recalculated = true;
                }
            }

            if recalculated {
                current = if previous.is_some() {
                    previous
                } else if level_type == LevelType::Ask {
                    self.best_trailing_buy_stop.clone()
                } else {
                    self.best_trailing_sell_stop.clone()
                };
            } else {
                previous = Some(current_level.clone());
                current = self.get_next_trailing_stop_level(current_level.clone())?;
            }
        }
        Ok(())
    }

    pub fn add_order(&mut self, order: &Order) -> Result<LevelUpdate, ErrorCode> {
        let update_type = UpdateType::Update;
    
        let (level_container, update_type) = if order.is_buy() {
            self.bids.as_ref().ok_or(ErrorCode::DefaultError)?
                .find_node_by_price(order.price)
                .map_or((self.add_level(order)?, UpdateType::Add), |lvl| (lvl, update_type))
        } else {
            self.asks.as_ref().ok_or(ErrorCode::DefaultError)?
                .find_node_by_price(order.price)
                .map_or((self.add_level(order)?, UpdateType::Add), |lvl| (lvl, update_type))
        };

        level_container.try_borrow_mut()
            .map_err(|_| ErrorCode::DefaultError)
            .and_then(|mut level_node| {
                let mut level = level_node.level;
                level.add_volumes(order);
                level.orders.push_back(*order); // Assuming Order: Clone
                Ok(LevelUpdate {
                    update_type,
                    update: level, // Assuming Level: Clone, or implement logic to create a new Level instance
                    top: self.is_top_of_book(order),
                })
            })
    }


    #[cfg(feature = "macro")]
    pub fn reduce_order(&mut self, order: &Order, quantity: u64, hidden: u64, visible: u64) -> Result<LevelUpdate, ErrorCode> {
        with_level!(order, |level: &mut RefMut<Level>| {
            level.subtract_volumes(order)?; // Assuming this method updates volumes and returns Result<(), ErrorCode>
    
            if order.leaves_quantity == 0 {
                level.orders.pop_current(order)?; // Assuming this operation might fail
            }
    
            if level.total_volume == 0 {
                self.delete_level(order)?;
            }
    
            Ok(LevelUpdate {
                update_type: if level.total_volume == 0 { UpdateType::Delete } else { UpdateType::Update },
                update: level.clone(), // Assuming Level: Clone
                top: self.is_top_of_book(order),
            })
        })
    }

    pub fn reduce_order(&mut self, order: &Order, quantity: u64, hidden: u64, visible: u64) -> Result<LevelUpdate, ErrorCode> {
        order.level.as_ref().ok_or(ErrorCode::DefaultError)
            .and_then(|level| {
                level
                    .try_borrow_mut()
                        .map_err(|_| ErrorCode::DefaultError)
                        .and_then(|mut level| {
                            level.subtract_volumes(order);
                        
                        if order.leaves_quantity == 0 {
                            level.orders.pop_current(order); // This operation is assumed to not fail
                        }
                        
                        if level.total_volume == 0 {
                            self.delete_level(order)?; // This operation can fail and is assumed to return Result<(), ErrorCode>
                        }
                        
                        Ok(LevelUpdate {
                            update_type: if level.total_volume == 0 { UpdateType::Delete } else { UpdateType::Update },
                            update: *level, // Assuming Level: Clone or a similar logic to create a Level instance
                            top: self.is_top_of_book(order),
                        })
            })
        })
    }

    #[cfg(feature = "macro")]
    pub fn delete_order(&mut self, order: &Order) -> Result<LevelUpdate, ErrorCode> {
        with_level!(order, |level: &mut RefMut<Level>| {
            level.subtract_volumes(order)?; // Assuming subtract_volumes now returns Result<(), ErrorCode>
            level.unlink_order(order)?; // Assuming unlink_order now returns Result<(), ErrorCode>
    
            if level.total_volume == 0 {
                self.delete_level(order)?;
            }
    
            Ok(LevelUpdate {
                update_type: if level.total_volume == 0 { UpdateType::Delete } else { UpdateType::Update },
                update: level.clone(), // Assuming Level: Clone
                top: self.is_top_of_book(order),
            })
        })
    }

    pub fn delete_order(&mut self, mut order: &mut Order) -> Result<LevelUpdate, ErrorCode> {
        order.level.as_ref().ok_or(ErrorCode::DefaultError)
            .and_then(|mut level| {
                level
                    .try_borrow_mut()
                    .map_err(|_| ErrorCode::DefaultError)
                    .and_then(|mut level| {
                        level.subtract_volumes(order);
                        level.unlink_order(&mut order);

                        if level.total_volume == 0 {
                            self.delete_level(order)?;
                        }
                        let update_type = if level.total_volume == 0 { UpdateType::Delete } else { UpdateType::Update };
                        Ok(LevelUpdate {
                            update_type,
                            update: *level, // Assuming Level: Clone
                            top: self.is_top_of_book(order),
                        })
                })
        })
    }

    pub fn reduce_stop_order(&mut self, order: &mut Order, quantity: u64, hidden: u64, visible: u64) -> Result<(), ErrorCode> {
        order.level.as_ref().ok_or(ErrorCode::DefaultError)
            .and_then(|mut level| {
                level
                    .try_borrow_mut()
                    .map_err(|_| ErrorCode::DefaultError)
                    .and_then(|mut level| {
                        level.subtract_volumes(order);
                        if order.leaves_quantity == 0 {
                            level.unlink_order(order);
                        }
                        if level.total_volume == 0 {
                            self.delete_stop_level(order)?;
                        }
                        Ok(())
                    }
                )
        })
    }

    #[cfg(feature = "macro")]
    pub fn delete_stop_order(&mut self, order: &Order) -> Result<(), ErrorCode> {
        with_level!(order, |level: &mut RefMut<Level>| {
            level.subtract_volumes_from_order(order.leaves_quantity, order.hidden_quantity(), order.visible_quantity)?;
            level.unlink_order(order)?;
    
            if level.total_volume == 0 {
                self.delete_stop_level(order)?;
            }
    
            Ok(())
        })
    }
    
    pub fn delete_stop_order(&mut self, order: &mut Order) -> Result<(), ErrorCode> {
        order.level.as_mut().ok_or(ErrorCode::DefaultError)
            .and_then(|mut level| {
                level
                    .try_borrow_mut()
                    .map_err(|_| ErrorCode::DefaultError)
                    .and_then(|mut level| {
                        level.subtract_volumes(order);
                        level.unlink_order(order);
                        if level.total_volume == 0 {
                            self.delete_stop_level(order)?;
                        }
                        Ok(()) 
                })
            }
        )
    }

    #[cfg(feature = "macro")]
    pub fn delete_trailing_stop_order(&mut self, order: &Order) -> Result<(), ErrorCode> {
        with_level!(order, |level: &mut RefMut<Level>| {
            level.subtract_volumes(order)?; // Assuming this method updates volumes and returns Result<(), ErrorCode>
            level.unlink_order(order)?; // Assuming this method unlinks the order and returns Result<(), ErrorCode>
    
            if level.total_volume == 0 {
                self.delete_trailing_stop_level(order)?;
            }
            Ok(())
        })
    }

    pub fn delete_trailing_stop_order(&mut self, order: &Order) -> Result<(), ErrorCode> 
    {
        order.level.as_ref().ok_or(ErrorCode::DefaultError)
            .and_then(|mut level| {
                level
                    .try_borrow_mut()
                    .map_err(|_| ErrorCode::DefaultError)
                    .and_then(|mut level| {
                    level.subtract_volumes(order); // Assuming this method updates volumes and returns Result<(), ErrorCode>
                    level.unlink_order(order); // Assuming this method unlinks the order and returns Result<(), ErrorCode>

                    if level.total_volume == 0 {
                        self.delete_trailing_stop_level(order)?;
                    }
                    Ok(())
                }
            )
        })
    }

    pub fn delete_stop_level(&mut self, order: &mut Order) -> Result<(), ErrorCode> {
        let level_ref = order.level_node.as_ref().ok_or(ErrorCode::DefaultError)?;
    
        // Determine which stop level collection to work with based on the order type.
        let stop_level_collection = if order.is_buy() {
            &mut self.best_buy_stop
        } else {
            &mut self.best_sell_stop
        };
    
        // If the current stop level matches the level to be deleted, update the collection.
        if let Some(stop_level) = stop_level_collection {
            if (*stop_level).eq(level_ref) {
                let mut borrowed_level = stop_level.try_borrow_mut().map_err(|_| ErrorCode::DefaultError)?;
                let next_best_level = borrowed_level.right.clone()
                    .or_else(|| borrowed_level.parent.as_ref().and_then(|weak| weak.upgrade().map(|parent| parent)));
    
                // Update the stop level collection with the next best level.
                *stop_level_collection = next_best_level;
            }
        }
    
        Ok(())
    }
}