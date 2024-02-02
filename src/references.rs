use std::ops::Deref;

use crate::order_book::order_book::OrderBook;

struct OBRef<'a> {
    order_book: &'a OrderBook<'a>,
}

impl<'a> Deref for OBRef<'a> {
    type Target = OrderBook<'a>;
    fn deref(&self) -> &Self::Target {
        self.order_book
    }
}


// A wrapper struct for a mutable reference to an OrderBook
struct OBMutRef<'a> {
    order_book: &'a mut OrderBook<'a>,
}

impl<'a> Deref for OBMutRef<'a> {
    type Target = OrderBook<'a>;
    
    fn deref(&self) -> &Self::Target {
        self.order_book
    }
}
