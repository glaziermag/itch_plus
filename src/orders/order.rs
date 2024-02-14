use core::fmt;

use crate::levels::indexing::{LevelNode, NodeHolder};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Buy // Assuming 'Buy' as a sensible default
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrderType {
    Buy,
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
    TrailingStopLimit,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market // Assuming 'Market' as a sensible default
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeInForce {
    IOD,
    FOK,
    // Other variants...
}

impl Default for TimeInForce {
    fn default() -> Self {
     TimeInForce::IOD // Assuming 'IO' as a sensible default
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    OK,
    SymbolDuplicate,
    SymbolNotFound,
    OrderBookDuplicate,
    OrderBookNotFound,
    OrderDuplicate,
    OrderNotFound,
    OrderIdInvalid,
    OrderTypeInvalid,
    OrderParameterInvalid,
    OrderQuantityInvalid,
    OrderCreationError,
    DummyError,
    OtherError(String),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&'static str> for ErrorCode {
    fn from(s: &'static str) -> Self {
        ErrorCode::OtherError(s.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    pub id: u64,
    pub symbol_id: u64,
    pub order_type: OrderType,
    pub order_side: OrderSide,
    pub price: u64,
    pub stop_price: u64,
    pub quantity: u64,
    pub executed_quantity: u64,
    pub leaves_quantity: u64,
    pub hidden_quantity: u64,
    pub visible_quantity: u64,
    pub time_in_force: TimeInForce,
    pub max_visible_quantity: u64,
    pub slippage: u64,
    pub trailing_distance: u64,
    pub trailing_step: u64,
    pub level_node: Option<NodeHolder<LevelNode>>,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            id: 0,
            symbol_id: 0,
            order_type: Default::default(),
            order_side: Default::default(),
            price: 0,
            stop_price: 0,
            quantity: 0,
            executed_quantity: 0,
            leaves_quantity: 0,
            time_in_force: Default::default(),
            max_visible_quantity: 0,
            slippage: 0,
            trailing_distance: 0,
            trailing_step: 0,
            hidden_quantity: todo!(),
            visible_quantity: todo!(),
            level_node: todo!(),
        }
    }
}

impl Order {
    pub fn validate(&self) -> Result<(), ErrorCode> {
        // Validate order Id
        if self.id == 0 {
            return Err(ErrorCode::OrderIdInvalid);
        }

        // Validate order quantity
        if self.quantity < self.leaves_quantity {
            return Err(ErrorCode::OrderQuantityInvalid);
        }
        if self.leaves_quantity == 0 {
            return Err(ErrorCode::OrderQuantityInvalid);
        }

        // Validate market order
        if self.is_market() {
            if !self.is_ioc() && !self.is_fok() {
                return Err(ErrorCode::OrderParameterInvalid);
            }
            if self.is_iceberg() {
                return Err(ErrorCode::OrderParameterInvalid);
            }
        }

        // Validate limit order
        if self.is_limit() {
            if self.is_slippage() {
                return Err(ErrorCode::OrderParameterInvalid);
            }
        }
        Ok(())
    }

    pub fn leaves_quantity(&self) -> u64 {
        self.leaves_quantity
    }

    pub fn is_market(&self) -> bool {
        self.order_type == OrderType::Market
    }

    pub fn hidden_quantity(&self) -> u64 {
        if self.leaves_quantity > self.max_visible_quantity {
            self.leaves_quantity - self.max_visible_quantity
        } else {
            0
        }
    }

    pub fn visible_quantity(&self) -> u64 {
        std::cmp::min(self.leaves_quantity, self.max_visible_quantity)
    }
}

impl Order {
    // Corresponds to the C++ constructor that accepts an Order
    pub fn new(order: &Order) -> Self {
        Self {
            id: todo!(),
            level_node: todo!(),
            symbol_id: todo!(),
            order_type: todo!(),
            order_side: todo!(),
            price: todo!(),
            stop_price: todo!(),
            quantity: todo!(),
            executed_quantity: todo!(),
            leaves_quantity: todo!(),
            hidden_quantity: todo!(),
            visible_quantity: todo!(),
            time_in_force: todo!(),
            max_visible_quantity: todo!(),
            slippage: todo!(),
            trailing_distance: todo!(),
            trailing_step: todo!(),
        }
    }
    pub fn is_limit(&self) -> bool {
        // Add logic to determine if the order is a limit order
        false
    }

    pub fn is_buy(&self) -> bool {
        // Add logic to determine if the order is a buy order
        false
    }

    pub fn is_fok(&self) -> bool {
        // Add logic to determine if the order is a Fill-or-Kill order
        false
    }

    pub fn is_iceberg(&self) -> bool {
        // Add logic to determine if the order is an iceberg order
        false
    }

    pub fn is_slippage(&self) -> bool {
        // Add logic to determine if the order is subject to slippage
        false
    }

    pub fn is_aon(&self) -> bool {
        // Add logic to determine if the order is an All-or-None order
        false
    }

    pub fn is_ioc(&self) -> bool {
        // Add logic to determine if the order is an Immediate-or-Cancel order
        false
    }

    // Check if the order is a trailing stop
    pub fn is_trailing_stop(&self) -> bool {
        matches!(self.order_type, OrderType::TrailingStop)
    }

    // Check if the order is a trailing stop limit
    pub fn is_trailing_stop_limit(&self) -> bool {
        matches!(self.order_type, OrderType::TrailingStopLimit)
    }

    // Returns a mutable reference to the next Order
    pub fn next(&self) -> Option<&Order> {
        self.next()
    }

    // Returns a mutable reference to the next Order
    pub fn next_mut(&self) -> Option<&mut Order> {
        self.next_mut()
    }

}
