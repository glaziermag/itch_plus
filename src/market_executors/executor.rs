

use crate::{order_book::order_book::BookOps, market_handler::Handler, levels::{indexing::{Tree, RcNode}}, orders::{order::{OrderNode, Order}}, references::Convertible};

pub trait Execution<'a> {
    fn activate_stop_order<B, C>(order_book: C, order_node: &OrderNode)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn activate_stop_limit_order<B, C, O>(order_book: C, order_node: &mut OrderNode)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn reduce_order(order_node_id: i64, quantity: u64, hidden: bool, visible: bool);
    fn match_order<B, C>(order_book: C, order: Order)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn remove_order(id: i64);
    fn calculate_matching_chain_single_level<B, C>(order_book: C, level: Level, price: u64, leaves_quantity: u64)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn calculate_matching_chain_cross_levels<B, C>(order_book: C, bid_level_node: Option<RcNode>, ask_level_node: Option<RcNode>)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn execute_matching_chain<B, C>(order_book: C, level: Level, price: u64, chain: u64)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn delete_order_recursive(executing_order_id: i64, flag1: bool, flag2: bool);
    fn activate_stop_orders_level<B, C>(order_book: C, stop_price: u64)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn activate_stop_orders<B, C>(order_book: C)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn recalculate_trailing_stop_price<B, C>(order_book: C, best_ask_or_bid: Level)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn activate_individual_stop_orders<B, O: OrderOps, C>(order_book: C, stop_level: Level, market_price: u64, orders: O)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn match_market<B, C>(order_book: C, order: Order)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn match_limit<B, C>(order_book: C, order: Order)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn update_level<B, C>(order_book: C, level: Level);
    fn match_order_book<B, H: Handler, C>(order_book: C, market_handler: H)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn add_market_order<B, H: Handler, C>(order_books: B, order: Order, matching: bool, recursive: bool, market_handler: H)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn add_limit_order<B, H: Handler, C>(order: Order, matching: bool, order_books: B, recursive: bool, market_handler: H)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn add_stop_order<B, H: Handler, O: OrderOps, C>(orders: O, order_books: B, order: Order, matching: bool, recursive: bool, market_handler: H)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn add_stop_limit_order<B, H: Handler, O: OrderOps, C>(order_books: B, orders: O, market_handler: H, order: Order, matching: bool, recursive: bool)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn modify_order(id: i64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool, flag3: bool);
    fn replace_order_internal(id: i64, new_id: i64, new_price: u64, new_quantity: u64, flag1: bool, flag2: bool);
    fn get_order_node(id: i64) -> Result<OrderNode<'a>, ErrorCode>;
    fn update_level_on_reduction<B, C>(order_book: C, order_node: OrderNode, quantity: u64, hidden: u64, visible: u64)
    where
        B: BookOps<'a>,
        C: Convertible<B>;
    fn subtract_level_volumes(level: Level, order_node: &OrderNode);
    fn unlink_order(level: Level, order_node: OrderNode);
}

pub struct MarketExecutor;

pub fn execute_matching_chain<E, H, B, T, C>(order_book: C, mut level_node: Option<RcNode>, price: u64, mut volume: u64) 
    where
        E: for<'a> Execution<'a>,
        H: Handler,
        B: for<'a> BookOps<'a>,
        T: for<'a> Tree<'a>,
        C: Convertible<B>
    {

    // the overhead of ref counting and whatnot not really needed except for the tree integrity it seems

    while volume > 0 {
        if let Some(current_level) = level_node {
            let mut executing_order = (*current_level.borrow_mut()).orders.front_mut();
          //  let mut executing_order = current_level.orders.front_mut();

            while volume > 0 {
                if let Some(order_node) = executing_order {
                    let quantity = if order_node.is_aon() {
                        order_node.leaves_quantity
                    } else {
                        std::cmp::min(order_node.leaves_quantity, volume)
                    };

                    H::on_execute_order(&order_node.order, price, quantity);
                    // Switch to the next price level
                    order_book.as_ref().update_last_price(order_node.order, price);
                    order_book.as_ref().update_matching_price(order_node.order, price);
                    
                    order_node.executed_quantity += quantity;
                    // Reduce the executing order in the order book
                    E::reduce_order(order_node.id, quantity, true, false);

                    volume -= quantity;
                    executing_order = order_node.next_mut();
                } else {
                    break;
                }
            }
            // Assuming `get_next_level_node` returns an Level
            if let Some(next_level) = T::get_next_level_node(current_level) {
                level_node = Some(next_level);
            } else {
                break;
            } 
        } else {
            break;
        }
    }
}

