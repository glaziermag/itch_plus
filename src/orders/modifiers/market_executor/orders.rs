use std::cmp::min;

use crate::{orders::order::{ErrorCode, Order, OrderType}, market_executors::executor::MarketExecutor};


trait Orders {
    fn modify_order(&self, id: u64, new_price: u64, new_quantity: u64, mitigate: bool, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn delete_order_recursive(&self, id: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn modify_order_volumes(&self, id: u64, quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn reduce_order(&self, id: u64, quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn mitigate_order(&self, id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode>;
    fn replace_order_id(&self, symbols: Vec<u64>, id: u64, new_id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode>;
    fn replace_order(&self, id: u64, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode>;
    fn replace_order_internal(&mut self, id: u64, new_id: u64, new_price: u64, new_quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode>;

}

impl Orders for MarketExecutor {

    fn mitigate_order(&self, id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> {
        self.modify_order(id, new_price, new_quantity, true, true, false)
    }

    fn replace_order_id(&self, symbols: Vec<u64>,id: u64, new_id: u64, new_price: u64, new_quantity: u64) -> Result<(), ErrorCode> {
        self.replace_order_internal(symbols, id, new_id, new_price, new_quantity, false)
    }

    fn modify_order(&self, id: u64, new_price: u64, new_quantity: u64, mitigate: bool, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        if id == 0 {
            return Err(ErrorCode::OrderIdInvalid);
        }
        if new_quantity == 0 {
            return Err(ErrorCode::OrderQuantityInvalid);
        }

        // Retrieve and modify the order
        let mut order_node = self.get_order_node(&id).ok_or(ErrorCode::OrderNotFound)?;
        
        if order_node.order_type != OrderType::Limit {
            return Err(ErrorCode::OrderTypeInvalid);
        }

        // Apply the modifications
        order_node.price = new_price;
        order_node.quantity = new_quantity;
        order_node.leaves_quantity = new_quantity;

        // In-Flight Mitigation (IFM) logic
        if mitigate {
            order_node.leaves_quantity = new_quantity.saturating_sub(order_node.executed_quantity);
        }

        // Handle the updated order
        if order_node.leaves_quantity > 0 {
            // Handle the case where the order is still active
            // e.g., market_handler.on_update_order(&order_node.order);
        } else {
            // Handle the case where the order is now fully executed
            // e.g., market_handler.on_delete_order(&order_node.order);
            self.remove_order(&id);
        }

        // Implement matching logic, if required
        if matching && !recursive {
            // Implement logic for matching orders after modification
            // This might involve interacting with an order book or a matching engine
        }

        Ok(())
    }

    fn delete_order_recursive(&self, id: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        // Validate parameters
        if id == 0 {
            return Err(ErrorCode::OrderIdInvalid);
        }

        // get the order to delete
        let order_node = self.get_order_node(&id);

        // get the valid order book for the order
        let order_book = self.order_books.get(&order_node.symbol_id).ok_or(ErrorCode::OrderBookNotFound)?;

        // Delete the order from the order book
        match order_node.order_type {
            OrderType::Limit => {
                self.update_level(self.order_books.get(&order_node.symbol_id).ok_or(ErrorCode::OrderBookNotFound), 
                self.order_books.get(&order_node.symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.delete_order(order_node));
            },
            OrderType::Stop | OrderType::StopLimit => {
                self.order_books.get(&order_node.symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.delete_stop_order(order_node);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                self.order_books.get(&order_node.symbol_id).ok_or(ErrorCode::OrderBookNotFound)?.delete_trailing_stop_order(order_node);
            },
            _ => return Err(ErrorCode::OrderTypeInvalid),
        };

        // Call the corresponding handler
        // market_handler.on_delete_order_node(order_node);

        // Erase the order
        self.remove_order(&id);

        // Release the order
        // order_pool.release(order_node);

        // Automatic order matching
        if matching && !recursive {
            self.match_order_book(order_book, self.market_handler);
        }

        order_book.reset_matching_price();

        // Reset matching price
        Ok(())
    }

    fn modify_order_volumes(&self, id: u64, quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        if id == 0 {
            return Err(ErrorCode::OrderIdInvalid);
        }
        if quantity == 0 {
            return Err(ErrorCode::OrderQuantityInvalid);
        }

        // Retrieve the order node
        let order_node = self.get_order_node(&id);

        // Since MarketExecutor deals with limit orders, assume it has its way of handling them.
        // Here, we focus on the logic specific to reducing a limit order.

        let quantity_to_reduce = std::cmp::min(quantity, order_node.leaves_quantity);
        order_node.leaves_quantity -= quantity_to_reduce;

        if order_node.leaves_quantity > 0 {
            // Handle the case where the order is partially filled
            // market_handler.on_update_order(&order_node.order);
            // Any additional logic for updating the order goes here
        } else {
            // Handle the case where the order is fully executed
            // market_handler.on_delete_order(&order_node.order);
            self.remove_order(&id); // Remove the order from the collection
            // Any additional logic for removing the order goes here
        }

        // Matching logic, if required
        if matching && !recursive {
            // Implement the logic for matching orders after reduction
            // This might involve interacting with an order book or a matching engine
        }

        Ok(())
    }

    fn reduce_order(&self, id: u64, quantity: u64, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        if id == 0 {
            return Err(ErrorCode::OrderIdInvalid);
        }
        if quantity == 0 {
            return Err(ErrorCode::OrderQuantityInvalid);
        }
        let mut order_book = self.get_order_book(&id);
        let mut order_node = self.get_order_node(id);

        let quantity = min(quantity, order_node.leaves_quantity);
        order_node.leaves_quantity -= quantity;

        let hidden = order_node.order.hidden_quantity();
        let visible = order_node.order.visible_quantity();

        // Update the order or delete the empty order
        if order_node.leaves_quantity > 0 {
            // market_handler.on_update_order(&order_node.order);
            let order_node = order_node.new(&order_node.order);

            // market order book into full order book
            match order_node.order_type {
                OrderType::Limit => {
                    order_book.reduce_trailing_stop_order(order_node, quantity, hidden, visible);
                },
                OrderType::Stop | OrderType::StopLimit => {
                    order_book.reduce_trailing_stop_order(order_node, quantity, hidden, visible);
                },
                OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                    order_book.reduce_trailing_stop_order(order_node, quantity, hidden, visible);
                },
                _ => return Err(ErrorCode::OrderTypeInvalid),
            };
        } else {
            // market_handler.on_delete_order(&order_node.order);
            match order_node.order_type {
                OrderType::Limit => {
                    self.update_level(order_book.reduce_order(order_node, quantity, hidden, visible));
                },
                OrderType::Stop | OrderType::StopLimit => {
                    order_book.reduce_stop_order(order_node, quantity, hidden, visible);
                },
                OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                    order_book.reduce_trailing_stop_order(order_node, quantity, hidden, visible);
                },
                _ => return Err(ErrorCode::OrderTypeInvalid),
            }

            // Erase the order
            self.remove_order(&id);
            // Release the order, assuming we have an order pool with a release method
            // order_pool.release(order_node);
        }

        if matching && !recursive {
            self.match_order_book(order_book, self.market_handler);
        }
        
        order_book.reset_matching_price();
        

        Ok(())
    }

    fn replace_order(&self, id: u64, order: Order, matching: bool, recursive: bool) -> Result<(), ErrorCode> {
        // Delete the previous order by Id
        let result = self.delete_order(id.try_into(), true, false);
        if let Err(error) = result {
            return Err(error);
        }
        self.add_order(order, matching, recursive)
    }

    fn replace_order_internal(
        &mut self,
        id: u64,
        new_id: u64,
        new_price: u64,
        new_quantity: u64,
        matching: bool,
        recursive: bool,
    ) -> Result<(), ErrorCode> {
        // Validate parameters 
        if id == 0 || new_id == 0 || new_quantity == 0 {
            return Err(ErrorCode::OrderIdInvalid);
        }

        // Retrieve the order to replace
        let order_node = self.get_order(&id).ok_or(ErrorCode::OrderNotFound)?;
        if !order_node.is_trailing_stop() && !order_node.is_trailing_stop_limit() {
            return Err(ErrorCode::OrderTypeInvalid);
        }

        // Retrieve the valid order book for the order
        let mut order_book = self.get_order_book(order_node.symbol_id)?;

        // Delete the trailing stop order from the order book
        order_book.delete_trailing_stop_order(order_node)?;

        // Replace the order
        let new_order = Order {
            id: new_id,
            price: new_price,
            quantity: new_quantity,
            executed_quantity: 0,
            leaves_quantity: new_quantity,
            ..*order_node // Clone other fields from the existing order
        };

        // Insert the new order into the manager's collection
        if self.orders.insert(new_id, new_order).is_some() {
            return Err(ErrorCode::OrderDuplicate);
        }

        // Add the new order into the order book
        order_book.add_trailing_stop_order(&self.orders[&new_id])?;

        // Handle automatic order matching if required
        if matching && !recursive {
            self.match_order_book(&mut order_book)?;
        }

        // Reset matching price in the order book
        order_book.reset_matching_price();

        Ok(())
    }
}