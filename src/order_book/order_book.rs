

use std::{cell::{RefCell}, cmp::{max, min}, rc::Rc};
use crate::{levels::{indexing::{LevelNode, Holder, TreeOps, TreeRemoval}, level::{Level, LevelOps, LevelType, LevelUpdate, PopCurrent, UpdateType}}, market_executors::executor::Execution, market_handler::Handler, orders::{order::{ErrorCode, Order, OrderType}, orders::OrderOps}};

#[derive(Debug)]
pub enum OrderBookError {
    OrderNotFound,
    LevelNotFound,
}

#[derive(Default)]
pub struct OrderBook {
    pub best_bid: Option<Holder<LevelNode>>,
    pub best_ask: Option<Holder<LevelNode>>,
    pub bids: Option<Holder<LevelNode>>,
    pub asks: Option<Holder<LevelNode>>,

    pub best_buy_stop: Option<Holder<LevelNode>>,
    pub best_sell_stop: Option<Holder<LevelNode>>,
    pub buy_stop: Option<Holder<LevelNode>>,
    pub sell_stop: Option<Holder<LevelNode>>,

    pub(crate) last_bid_price: u64,
    pub(crate) last_ask_price: u64,
    pub(crate) matching_bid_price: u64,
    pub(crate) matching_ask_price: u64,

    pub best_trailing_buy_stop: Option<Holder<LevelNode>>,
    pub best_trailing_sell_stop: Option<Holder<LevelNode>>,
    pub trailing_buy_stop: Option<Holder<LevelNode>>,
    pub trailing_sell_stop: Option<Holder<LevelNode>>,
    pub trailing_bid_price: u64,
    pub trailing_ask_price: u64,
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
        }
    }


    //#[cfg(feature = "experimental_level_changes")]
    pub fn reduce_trailing_stop_order(&mut self, order_book: &mut OrderBook, order: &mut Order, quantity: u64, hidden: u64, visible: u64)
    {
        // Assuming we have a way to get a mutable reference to an order and its level.
        // Update the price level volume
        order
            .level
            .ok_or(ErrorCode::OtherError("Level is missing for the order".to_string()))
            .and_then(|mut level| level.subtract_volumes(order))
            .and_then(|level| level.conditional_unlink_order(order))
            .and_then(|level| level.process_level(order_book, order));
    }

    // Method to get the best trailing buy stop level
    pub fn best_trailing_buy_stop(&self) -> Option<Holder<LevelNode>> {
        self.best_trailing_buy_stop.clone()
    }

    // Method to get the best trailing sell stop level
    pub fn best_trailing_sell_stop(&self) -> Option<Holder<LevelNode>> {
        self.best_trailing_sell_stop.clone()
    }

    pub fn get_trailing_buy_stop_level(&mut self, price: &u64) -> Option<Holder<LevelNode>> {
        //(self.trailing_buy_stop.expect("best trailing buy stop failed")).get(price)
        self.trailing_buy_stop.clone().expect("node not retrieved").find(price)
    }

    // Method to get the trailing sell stop level
    pub fn get_trailing_sell_stop_level(&mut self, price: &u64) -> Option<Holder<LevelNode>>                                         
    {
        self.trailing_sell_stop.clone().expect("node not retrieved").find(price)
    }

    pub fn get_next_trailing_stop_level(&mut self, level_node: Holder<LevelNode>) -> Option<Holder<LevelNode>>                                         
    {  
        if level_node.try_borrow().level.is_bid() {
            // Find the next level in reverse order in _trailing_sell_stop
            self.trailing_sell_stop.clone().expect("best trailing sell stop failed").get_next_lower_level(level_node)
        } else {
            // Find the next level in normal order in _trailing_buy_stop
            self.trailing_buy_stop.clone().expect("best trailing buy stop failed").get_next_higher_level(level_node)
        }
    }

    pub fn get_next_level_node(&self, level_node: Holder<LevelNode>) -> Option<Holder<LevelNode>>                                  
    {
        if level_node.try_borrow().level.is_bid() {
            // For a bid, find the next lower level
            self.bids.clone().expect("bids not retrieved").get_next_lower_level(level_node)
        } else {
            // For an ask, find the next higher level
            self.asks.clone().expect("asks not retrieved").get_next_higher_level(level_node)
        }
    }

    pub fn delete_trailing_stop_level(&mut self, order: &Order)                                           
    {
        // remove panicking behavior from code
        let level_node = order.level_node.expect("level node not found");
        
        if order.is_buy() {
            // Update the best trailing buy stop order price level
            // remove panicking behavior from code
            let best_stop = self.best_trailing_buy_stop.expect("best stop not retrieved");
            let price: u64;
            if best_stop == level_node {
                let borrow_stop = best_stop.try_borrow();
                price = borrow_stop.level.price;
                self.best_trailing_buy_stop = if borrow_stop.right.is_none() {
                    borrow_stop.right
                } else {
                    borrow_stop.parent
                }
            }
            // Erase the price level from the trailing buy stop orders collection
            self.best_trailing_buy_stop.expect("trailing buy stop not retieved").remove(price);
        } else {
            // Update the best trailing sell stop order price level
            // remove panicking behavior from code
            let best_stop = self.best_trailing_sell_stop.expect("best stop not retrieved");
            let price: u64;
            if best_stop == level_node {
                let borrow_stop = best_stop.try_borrow();
                price = borrow_stop.level.price;
                self.best_trailing_sell_stop = if borrow_stop.left.is_none() {
                    borrow_stop.left
                } else {
                    borrow_stop.parent
                }
            }
            // Erase the price level from the trailing sell stop orders collection
            self.trailing_sell_stop.expect("trailing sell stop not retieved").remove(price);
        }
        // Release the price level
        // self.level_pool.releaselevel_node.try_borrow().level.price)
    }

    pub fn add_trailing_stop_level(&mut self, order: &Order) -> Option<Holder<LevelNode>> {
        let (price, level_node) = if order.is_buy() {
            let level_node = Holder(Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order.stop_price)))));
            (order.stop_price, level_node)
        } else {
            let level_node = Holder(Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order.stop_price)))));
            (order.stop_price, level_node)
        };
        
        if order.is_buy() {
            self.trailing_buy_stop.insert(level_node);
            // Update the best trailing buy stop order price level
            if self.best_trailing_buy_stop.is_none() || level_node.try_borrow().level.price < self.best_trailing_buy_stop.expect("best trailing buy stop failed").try_borrow().level.price {
                self.best_trailing_buy_stop = Some(level_node);
            }
        } else {
            self.trailing_sell_stop.insert(level_node);
            // Update the best trailing sell stop order price level
            if self.best_trailing_sell_stop.is_none() || level_node.try_borrow().level.price < self.best_trailing_sell_stop.expect("best trailing sell stop failed").try_borrow().level.price {
                self.best_trailing_sell_stop = Some(level_node);
            }
        }
        Some(level_node)
    }

    pub fn best_buy_stop(&self) -> Option<Holder<LevelNode>> 
    {
        self.best_buy_stop.clone()
    }

    // Method to get the best sell stop level
    pub fn best_sell_stop(&self) -> Option<Holder<LevelNode>> 
    {
        self.best_sell_stop.clone()
    }

    pub fn add_stop_level(&mut self, order: &Order) -> Option<Holder<LevelNode>> 
    {
        // Determine the level type and price based on the order node
        // Determine the price and create a level node
        let level_option = if order.is_buy() {
            Holder(Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Ask, order.stop_price)))))
        } else {
            Holder(Rc::new(RefCell::new(LevelNode::from(Level::with_price(LevelType::Bid, order.stop_price)))))
        };

        let level_node = level_option;

        if order.is_buy() {
            self.buy_stop.insert(level_option);
            // remove panicking behavior from code
            let best_stop = self.best_buy_stop.expect("best stop");
            if self.best_buy_stop.is_none() || level_node.try_borrow().level.price < best_stop.try_borrow().level.price {
                self.best_buy_stop = Some(level_option);
            }
        } else {
            self.sell_stop.insert(level_option);
            // remove panicking behavior from code
            let best_stop = self.best_sell_stop.expect("best stop");
            if self.best_sell_stop.is_none() || level_node.try_borrow().level.price < best_stop.try_borrow().level.price {
                self.best_sell_stop = Some(level_option);
            }
        }
        Some(level_option)
    }

    pub fn create_and_insert_level(&mut self, price: u64, level_type: LevelType) -> Option<Holder<LevelNode>> 
    {
        // Create a new price level based on the provided level type
        // Insert the price level into the appropriate collection based on level type
        let new_node = Holder(Rc::new(RefCell::new(LevelNode::from(Level::with_price(level_type, price)))));
        match level_type {
            LevelType::Bid => {
                if let Some(bids_root) = self.bids {
                    self.bids.insert(new_node);
                } else {
                    // Handle the case where bids tree is empty
                    self.bids = Some(new_node);
                }
            },
            LevelType::Ask => {
                if let Some(asks_root) = self.asks {
                    self.asks.insert(new_node);
                } else {
                    // Handle the case where bids tree is empty
                    self.asks = Some(new_node);
                }
            },
        }
        Some(new_node)
    }

    pub fn add_stop_order(&mut self, order: &Order) 
    {
        // Find the price level for the order
        let level_node = if order.is_buy() {
            self.buy_stop.expect("buy stop not retrieved").find_node_by_price(order.stop_price)
        } else {
            self.sell_stop.expect("sell stop not retrieved").find_node_by_price(order.stop_price)
        };

        let binding = match level_node {
            Some(level) => level_node,
            None => {
                self.add_stop_level(order)
            },
        };

        if let Some(level_node) = binding {
            let mut level = level_node.try_borrow().level;
            level.add_volumes(order);
            // Link the new order to the orders list of the price level
            level.orders.push_back(*order); 
            order.level_node = Some(level_node)
        } else {
        // let level_node = level_node.try_borrow().level;
            order.level_node = level_node
        }
    }

    pub fn add_trailing_stop_order(&mut self, order: &Order) 
    {
        let level_node = if order.is_buy() {
            self.get_trailing_buy_stop_level(&order.stop_price)
                .or_else(|| {
                self.add_trailing_stop_level(order)
            })// Clones the Arc, not the Level
        } else {
            self.get_trailing_sell_stop_level(&order.stop_price)
                .or_else(|| {
                self.add_trailing_stop_level(order)
            }) // Clones the Arc, not the Level
        };

        let mut level = level_node.expect("tree operation failed").try_borrow().level;
        // Update the price level volume
        level.add_volumes(order);

        // Link the new order to the orders list of the price level
        // check for correctness
        level.link_order(order);

        // Unlink the empty order from the orders list of the price level
        level.orders.push_back(*order);

        order.level_node.expect("order node level node expected").try_borrow().level = level;
    }

    pub fn delete_level(&mut self, order: &Order)                                             
    {
        // remove panicking behavior from code
        let level_node = order.level_node.expect("order node level not retrieved");
        if order.is_buy() {
            // remove panicking behavior from code
            let best_bid = self.best_bid.expect("best bid not retrieved");
            let price: u64;
            if best_bid == level_node {
                // Update the best bid price level
                let borrowed_best = best_bid.try_borrow_mut();
                self.best_bid = if borrowed_best.left.is_some() {
                    borrowed_best.left
                } else if borrowed_best.parent.is_some() {
                    borrowed_best.parent
                } else {
                    borrowed_best.right
                };
                let price: u64 = self.bids.expect("asks not retrieved").try_borrow().level.price;
                self.bids.expect("bids not retrieved").remove(price);
            }
            // Erase the price level from the bid collection
        } else {
            // remove panicking behavior from code
            let best_ask: Holder<LevelNode> = self.best_ask.expect("best bid not retrieved");
            if best_ask == level_node {
                let borrowed_best = best_ask.try_borrow_mut();
                // Update the best bid price level
                self.best_ask = if borrowed_best.left.is_some() {
                    borrowed_best.left
                } else if borrowed_best.parent.is_some() {
                    borrowed_best.parent
                } else {
                    borrowed_best.right
                };
                let price: u64 = self.asks.expect("asks not retrieved").try_borrow().level.price;
                self.asks.expect("asks not retrieved").remove(price);
            }
        }
    }

    pub fn add_level(&mut self, order: &Order) -> Option<Holder<LevelNode>> 
    {
        let level_node = self.create_and_insert_level(order.price, if order.is_buy() { LevelType::Bid } else { LevelType::Ask });
        // remove panicking behavior from code
        let node_borrow = level_node.expect("add level node borrow").try_borrow();
        
        if order.is_buy() {
            // remove panicking behavior from code
            if self.best_bid.is_none() || node_borrow.level.price > self.best_bid.expect("best bid failed").try_borrow().level.price {
                self.best_bid = level_node.clone()
            }
        } else {
            // remove panicking behavior from code
            if self.best_ask.is_none() || node_borrow.level.price < self.best_ask.expect("best ask failed").try_borrow().level.price {
                self.best_ask = level_node.clone()
            }
        }
        level_node.clone()
    }

    pub fn best_ask(& self) -> Option<Holder<LevelNode>>                              
    {
        self.best_ask.clone()
    }

    pub fn best_bid(&self) -> Option<Holder<LevelNode>>                                   
    {
        self.best_bid.clone()
    } 

    pub fn get_bid(&mut self, price: u64) -> Option<Holder<LevelNode>>                
    {
       // let price: u64 = self.bids.expect("asks not retrieved").try_borrow().level.price;
        self.bids.expect("bids not retrieved").find_node_by_price(self.bids.expect("asks not retrieved").try_borrow().level.price)
    }

    pub fn get_ask(&mut self, price: u64) -> Option<Holder<LevelNode>>                                
    {
       // let price: u64 = self.asks.expect("asks not retrieved").try_borrow().level.price;
        self.asks.expect("asks not retrieved").find_node_by_price(self.asks.expect("asks not retrieved").try_borrow().level.price)
    }

    pub fn get_market_trailing_stop_price_ask(&mut self) -> u64                                      
    { 
        let last_price = self.last_ask_price;
        let best_price = self.best_ask.map_or(u64::MAX, |ask_node| ask_node.try_borrow().level.price);
        std::cmp::max(last_price, best_price)
    }

    pub fn get_market_trailing_stop_price_bid(&mut self) -> u64                                           
    {
        let last_price = self.last_bid_price;
        let best_price = if self.best_bid.is_some() {
            // remove panicking behavior from code
            self.best_bid.expect("best bid").try_borrow().level.price
        } else {
            0
        };
        std::cmp::min(last_price, best_price)
    }

    pub fn is_top_of_book(&mut self, order: &Order) -> bool                                          
    {
        if let Some(level_node) = order.level_node {
            return match order.is_buy() {
                true => {
                    // remove panicking behavior from code
                    self.best_bid.expect("best bid").try_borrow().level.price == level_node.try_borrow().level.price
                },
                false => {
                    // remove panicking behavior from code
                    self.best_ask.expect("best ask").try_borrow().level.price == level_node.try_borrow().level.price
                },
            };
        }
        false
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

    pub fn get_market_ask_price(&self) -> u64                                         
    {
        let best_price = if self.best_ask.is_some() {
            // remove panicking behavior from code
            self.best_ask.expect("market ask price").try_borrow().level.price
        } else {
            u64::MAX
        };
        min(best_price, self.matching_ask_price)
    }

    pub fn get_market_bid_price(&self) -> u64                                          
    {
        let best_price = if self.best_bid.is_some() {
            // remove panicking behavior from code
            self.best_bid.expect("market bid price").try_borrow().level.price
        } else {
            0
        };
        max(best_price, self.matching_bid_price)
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

    pub fn calculate_trailing_stop_price(&mut self, order: &Order) -> u64 
    {
        // Get the current market price
        let market_price = if order.is_buy() {
            self.get_market_trailing_stop_price_ask()
        } else {
            self.get_market_trailing_stop_price_bid()
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

    pub fn recalculate_trailing_stop_price<E>(&mut self, level_node: Option<Holder<LevelNode>>)
    where
        E: Execution<E> + Handler + OrderOps,
    {
        let mut new_trailing_price;

        let level_type = level_node.expect("level type needed").try_borrow().level.level_type;

        // Skip recalculation if market price goes in the wrong direction
        match level_type {
            LevelType::Ask => {
                let old_trailing_price = self.trailing_ask_price;
                new_trailing_price = self.get_market_trailing_stop_price_ask();
                if new_trailing_price >= old_trailing_price {
                    return;
                }
                self.trailing_ask_price = new_trailing_price;
            },
            LevelType::Bid => {
                let old_trailing_price = self.trailing_bid_price;
                new_trailing_price = self.get_market_trailing_stop_price_bid();
                if new_trailing_price <= old_trailing_price {
                    return;
                }
                self.trailing_bid_price = new_trailing_price;
            },
        }

        // Recalculate trailing stop self.orders
        let mut current = match level_type {
            LevelType::Ask => {
                self.best_trailing_buy_stop
            },
            LevelType::Bid => {
                self.best_trailing_sell_stop
            }
        };

        let mut previous: Option<Holder<LevelNode>> = None;

        while let Some(current_level) = current {
            let mut recalculated = false;
            let mut node = current_level.try_borrow().level.orders.front();

            while let Some(order) = node {
                let old_stop_price = order.stop_price;
                let new_stop_price = self.calculate_trailing_stop_price(order);

                // Update and re-add order if stop price changed
                if new_stop_price != old_stop_price {
                    self.delete_trailing_stop_order(&order);
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
                    E::on_update_order(&order);
                    self.add_trailing_stop_order(&order);
                    recalculated = true;
                }
                let next_order = order.next();
                node = next_order;
            }

            if recalculated {
                let current = if let Some(prev) = previous {
                    Some(prev) 
                } else if level_type == LevelType::Ask {
                    self.best_trailing_buy_stop
                } else {
                    self.best_trailing_sell_stop
                };
            } else {
                previous = current;
                current = self.get_next_trailing_stop_level(current_level);
            }
        }
    }


    pub fn add_order(&mut self, order: &Order) -> LevelUpdate 
    {
        let mut update_type = UpdateType::Update;
        // Find the price level for the order
        let mut existing_level = if order.is_buy() {
            self.bids.expect("bids not retrieved").find_node_by_price(order.price)
        //  self.bids.expect("order book bids")).get(&order.price)
        } else {
            self.asks.expect("asks not retrieved").find_node_by_price(order.price)
        //  self.asks.expect("order book asks")).get(&order.price)
        };

        let binding: Option<Holder<LevelNode>>;
        if let None = existing_level {
            binding = self.add_level(order);
            existing_level = binding;
            update_type = UpdateType::Add;
        }

        let level_node: Holder<LevelNode>;
        let mut level: Level;

        if let Some(level_node) = existing_level {
            level = level_node.try_borrow().level;
            level.add_volumes(order);
            level.orders.push_back(*order);
            order.level_node.expect("order node level not obtained").try_borrow().level = level;
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
            },
            top: self.is_top_of_book(order),
        }
    }

    pub fn reduce_order(&mut self, mut order: &Order, quantity: u64, hidden: u64, visible: u64) -> LevelUpdate 
    {
        let mut update_type = UpdateType::Update;
        let mut level_update: LevelUpdate;

        // remove panicking behavior from code
        let mut level_node = order.level_node.expect("level node not retrieved from order node");
        let mut level = level_node.try_borrow().level;
        level.total_volume -= quantity;
        level.hidden_volume -= hidden;
        level.visible_volume -= visible;

        if order.leaves_quantity == 0 {
            //self.unlink_order(level, order)
            level.orders.pop_current(&order);
        }

        if level.total_volume == 0 {
            // Clear the price level cache in the given order
        self.delete_level(order);
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
            },
            top: self.is_top_of_book(order),
        }
    }

    pub fn delete_order(&mut self, order: &Order) -> LevelUpdate 
    {
        // remove panicking behavior from code
        let mut level_node = order.level_node.expect("level node not retrieved from order node");
        let mut level = level_node.try_borrow().level;
        
        // Update the price level volume
        level.subtract_volumes(order);

        // Unlink the empty order from the orders list of the price level
        level.unlink_order(order);

        let mut update_type = UpdateType::Update;
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
        self.delete_level(order);
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
                
            },
            top: self.is_top_of_book(order),
        }

        
    }

    pub fn reduce_stop_order(&mut self, order: &Order, quantity: u64, hidden: u64, visible: u64) 
    {
        // Find the price level for the order
        // remove panicking behavior from code
        let mut level = order.level_node.expect("level node not retrieved from order node").try_borrow().level;

        // Update the price level volume
        level.total_volume -= quantity;
        level.hidden_volume -= hidden;
        level.visible_volume -= visible;
        // Unlink the empty order from the orders list of the price level
        if order.leaves_quantity == 0 {
            // Assuming pop_current is a function that removes an order based on Some criteria and returns an Option<order / Order />
            level.orders.pop_current(&order);
        }
        // Delete the empty price level
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_stop_level(order);
        };
    }

    pub fn delete_stop_order(&mut self, order: &Order) 
    {    
        // Update the price level volume
        // Find the price level for the order
        // remove panicking behavior from code
        let mut level = order.level_node.expect("level node not retrieved from order node").try_borrow().level;

        level.total_volume -= order.leaves_quantity;
        level.hidden_volume -= order.hidden_quantity();
        level.visible_volume -= order.visible_quantity;

        // Unlink the empty order from the orders list of the price level
        level.orders.pop_current(&order);

        // Delete the empty price level
        if level.total_volume == 0 {
            self.delete_stop_level(order);
        }
    }


    pub fn delete_trailing_stop_order(&mut self, order: &Order) -> Result<(), &'static str> 
    {
        // remove panicking behavior from code
        let mut level = order.level_node.expect("level node not retrieved from order node").try_borrow().level;
        
        // Update the price level volume
        // check for correctness with doubling up
        level.subtract_volumes(order);

        // Unlink the empty order from the orders list of the price level
        // let mut level = level.expect("order node level node not found")).level;
        level.orders.pop_current(&order); // Assuming each order has a unique identifier

        // Delete the empty price level
        if level.total_volume == 0 {
            // Clear the price level cache in the given order
            self.delete_trailing_stop_level(order);
        };
        Ok(())
    }

    pub fn delete_stop_level(&mut self, order: &Order) 
    {
        // remove panicking behavior from code
        let level_node = order.level_node.expect("order node level node not retrieved");

        if order.is_buy() {
            // Update the best buy stop order price level
            // remove panicking behavior from code
            let stop_level = self.best_buy_stop.expect("buy stop not found");
            let borrowed_level = stop_level;
            if stop_level == level_node {
                self.best_buy_stop = if borrowed_level.try_borrow().right.is_none() {
                    borrowed_level.try_borrow().right
                } else {
                    borrowed_level.try_borrow().parent
                }   
            }
            // Erase the price level from the buy stop orders collection
            self.best_buy_stop.expect("best buy stop not retrieved").remove(borrowed_level.try_borrow().level.price);
        // stop_level).remove(borrowed_level.try_borrow().price);
        } else {
            // remove panicking behavior from code
            let stop_level = self.best_sell_stop.expect("buy stop not found");
            let borrowed_level = stop_level;
            if stop_level == level_node  {
                // Update the best sell stop order price level
                self.best_sell_stop = if borrowed_level.try_borrow().right.is_none() {
                    borrowed_level.try_borrow().right
                } else {
                    borrowed_level.try_borrow().parent
                }
            }
            // Erase the price level from the sell stop orders collection
            self.best_sell_stop.expect("best sell stop not retrieved").remove(borrowed_level.try_borrow().level.price);
        // stop_level).remove(borrowed_level.try_borrow().price);
        }
    }
}