pub trait OrderCollection {
    type order_handle: OrderHandleTrait;
    fn get_order(&self, id: Self::Order) -> Option<&Self::Order>;
    fn add_order(&mut self, id: u64, order_handle: Self::Order);
}

impl<T> OrderCollection for HashMap<u64, T> 
    where
    T: OrderTrait
    {

    fn get_order(&self, id: u64) -> Option<&Self::Order> {
        self.get(&id)
    }

    fn add_order(&mut self, id: Self::Order, order_handle: Self::Order) {
        self.insert(id, order);
    }
}