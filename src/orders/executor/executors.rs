

use crate::{orders::order::{ErrorCode, OrderType}, market_executors::executor::MarketExecutor};


impl MarketExecutor {

    pub fn execute_order(&self, id: u64, price: u64, quantity: u64, matching: bool) -> Result<(), ErrorCode> {

        let mut order_node = Self::get_order_node(&id).ok_or(ErrorCode::OrderNotFound)?;

        let mut order_book = Self::get_order_book(&order_node.order.symbol_id);

        let quantity = std::cmp::min(quantity, order_node.leaves_quantity);
        H::on_execute_order_node(order_node, order_node.price, quantity);
        order_book.update_last_price(order_node.order, order_node.price);
        order_book.update_matching_price(order_node.order, order_node.price);

        let hidden = order_node.order.hidden_quantity();
        let visible = order_node.order.visible_quantity();
        order_node.executed_quantity += quantity;
        order_node.leaves_quantity -= quantity;

        let hidden_delta = hidden - order_node.order.hidden_quantity();
        let visible_delta = visible - order_node.order.visible_quantity();

        match order_node.order_type {
            OrderType::Limit => {
                self.update_level(order_book.reduce_order(order_node, quantity, hidden, visible));
            },
            OrderType::Stop | OrderType::StopLimit => { 
                order_book.reduce_stop_order(order_node, quantity, hidden_delta, visible_delta);
            },
            OrderType::TrailingStop | OrderType::TrailingStopLimit => {
                order_book.reduce_trailing_stop_order(order_node, quantity, hidden_delta, visible_delta);
            },
            OrderType::Buy => todo!(),
            OrderType::Market => todo!(),
        }

        if order_node.leaves_quantity > 0 {
            H::on_update_order(&order_node.order);
        } else {
            H::on_delete_order_node(order_node);
            Self::remove_order(&id);
            // order_pool.release(orders.get_mut(&id).ok_or(ErrorCode::OrderNotFound)?);
        }

        if matching {
            self.match_order_book(order_book, self.market_handler);
        }
        order_book.reset_matching_price();

        Ok(())
    }
}