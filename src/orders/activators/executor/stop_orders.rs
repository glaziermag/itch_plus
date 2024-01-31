use crate::{levels::indexing::RcNode, market_executors::executor::Execution, order_book::order_book::BookOps, orders::{order::{OrderNode, OrderType, TimeInForce}, orders::{OrderOps}}, market_handler::Handler, references::Convertible};


pub fn activate_stop_orders<E, O, B, C>(mut order_book: C, mut orders: O) -> bool 
where
    E: for<'a> Execution<'a>,
    O: OrderOps,
    B: for<'a> BookOps<'a>,
    C: Convertible<B>{

    let mut result = false;
    let mut stop = false;

    while !stop {
        stop = true;

        // Try to activate buy stop self.orders
        if E::activate_individual_stop_orders(order_book, order_book.best_buy_stop(), order_book.get_market_ask_price(), orders)
            || E::activate_individual_stop_orders(order_book, order_book.best_trailing_buy_stop(), order_book.get_market_ask_price(), orders) {
            result = true;
            stop = false;
        }
        let best_ask = order_book.best_ask();
        
        // Recalculate trailing buy stop self.orders
        E::recalculate_trailing_stop_price(order_book, best_ask);

        // Try to activate sell stop self.orders
        if E::activate_individual_stop_orders(order_book, order_book.best_sell_stop(), order_book.get_market_bid_price(), orders)
            || E::activate_individual_stop_orders(order_book, order_book.best_trailing_sell_stop(), order_book.get_market_bid_price(), orders) {
            result = true;
            stop = false;
        }

        let best_bid = order_book.best_bid();
        // Recalculate trailing sell stop self.orders
        E::recalculate_trailing_stop_price(order_book, best_bid);
    
    }
    result
}

pub fn activate_individual_stop_orders<E, B, O, A, C>(order_book: C, level_node: A, stop_price: u64, orders: O) -> bool 
where
    E: for<'a> Execution<'a>,
    B: for<'a> BookOps<'a>,
    O: OrderOps,
    A: for<'a> AsMut<RcNode<'a>>,
    C: Convertible<B>
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
                result |= E::activate_stop_limit_order(order_book, order_node, orders);
            },
            _ => panic!("Unsupported order type!"),
        }
        activating_order_node = next_activating_order_node;
    }
    result
}

pub fn activate_stop_order<E, B, O, A, C, H>(mut order_book: B, mut order_node: &OrderNode) -> bool 
where
    E: for<'a> Execution<'a>,
    B: for<'a> BookOps<'a>,
    O: OrderOps,
    A: for<'a> AsMut<RcNode<'a>>,
    C: Convertible<B>,
    H: Handler
    {
    
    // Delete the stop order from the order book
    if order_node.is_trailing_stop() || order_node.is_trailing_stop_limit() {
        B::delete_trailing_stop_order(order_node);
    } else {
        B::delete_stop_order(order_node);
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
    E::remove_order(&order_node.id);

    // Release the order, assuming we have an order pool with a release method
    // order_pool.release(order_node);
    true
}

pub fn activate_stop_limit_order<E, O, B, A, C, H>(mut order_book: B, mut order_node: &OrderNode, mut orders: O) -> bool 
where
    E: for<'a> Execution<'a>,
    B: for<'a> BookOps<'a>,
    O: OrderOps,
    A: for<'a> AsMut<RcNode<'a>>,
    C: Convertible<B>,
    H: Handler
    {
    // Delete the stop order from the order book
    if order_node.is_trailing_stop() || order_node.is_trailing_stop_limit() {
        B::delete_trailing_stop_order(order_node);
    } else {
        order_book.delete_stop_order(order_node);
    }

    order_node.order_type = OrderType::Limit;
    order_node.stop_price = 0;

    H::on_update_order(&order_node.order);

    E::match_limit(order_book, order_node.order);

    if order_node.leaves_quantity > 0 && !order_node.is_ioc() && !order_node.is_fok() {
        let level_update = order_book.add_order(order_node);
        E::update_level(order_book, level_update);
    } else {
        // Call the corresponding MarketHandler
        //H::on_delete_order(&order_node.order);
        E::remove_order(&order_node.order.id);
        // order_pool.release(order_node);
    }
    true
}
