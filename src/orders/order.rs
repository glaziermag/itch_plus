use core::fmt;

use crate::levels::{indexing::RcNode};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Buy // Assuming 'Buy' as a sensible default
    }
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub enum TimeInForce {
    IOC,
    FOK,
    // Other variants...
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::IOC // Assuming 'IOC' as a sensible default
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
    OrderNodeCreationError,
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

#[derive(PartialEq, Debug)]
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
    pub time_in_force: TimeInForce,
    pub max_visible_quantity: u64,
    pub slippage: u64,
    pub trailing_distance: u64,
    pub trailing_step: u64,
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

    pub fn is_limit(&self) -> bool {
        self.order_type == OrderType::Limit
    }

    pub fn is_trailing_stop(&self) -> bool {
        // Implementation example, adjust according to your needs
        self.order_type == OrderType::TrailingStop
    }

    pub fn is_trailing_stop_limit(&self) -> bool {
        // Implementation example, adjust according to your needs
        self.order_type == OrderType::TrailingStopLimit
    }

    pub fn is_buy(&self) -> bool {
        // Implementation example, adjust according to your needs
        self.order_side == OrderSide::Buy
    }

    pub fn is_fok(&self) -> bool {
        // Implementation example, adjust according to your needs
        self.time_in_force == TimeInForce::FOK
    }

    pub fn is_iceberg(&self) -> bool {
        // Implement based on your application's requirements
        false
    }

    pub fn is_slippage(&self) -> bool {
        // Implement based on your application's requirements
        false
    }

    pub fn is_aon(&self) -> bool {
        // Implement based on your application's requirements
        false
    }

    pub fn is_ioc(&self) -> bool {
        // Implement based on your application's requirements
        // For example, you might have an `order_type` field that you compare:
        // self.order_type == OrderType::IOC
        false
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

#[derive(PartialEq)]
pub struct OrderNode<'a> {
    // Nullable reference to Level
    pub order: Order,
    pub id: u64,
    pub symbol_id: u64,
    pub slippage: u64,
    pub price: u64,
    pub quantity: u64, 
    pub leaves_quantity: u64,
    pub executed_quantity: u64,
    pub hidden_quantity: u64,
    pub visible_quantity: u64,
    pub level_node: Option<RcNode<'a>>,
    pub order_type: OrderType,
    pub stop_price: u64,
    pub time_in_force: TimeInForce,
}

impl OrderNode<'_> {
    // Corresponds to the C++ constructor that accepts an Order
    pub fn new(order: Order) -> Self {
        Self {
            id: todo!(),
            symbol_id: todo!(),
            slippage: todo!(),
            price: todo!(),
            quantity: todo!(),
            leaves_quantity: todo!(),
            executed_quantity: todo!(),
            hidden_quantity: todo!(),
            visible_quantity: todo!(),
            order_type: todo!(),
            stop_price: todo!(),
            time_in_force: todo!(),
            order: todo!(),
            level_node: todo!(),
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

    // Returns a mutable reference to the next OrderNode
    pub fn next_mut(&self) -> Option<&mut OrderNode> {
        self.next_mut()
    }

}
