use std::rc::Rc;
use std::sync::Arc;
use std::boxed::Box;

use std::ops::Deref;

use crate::order_book::order_book::BookOps;

impl<B: for<'a> BookOps<'a>, C: Convertible<B>> Deref for C {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        self.as_ref() 
        // Return a reference to the underlying data
    }
}

pub trait Convertible<T> {
    fn into_owned(self) -> T;
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &mut T;
    fn into_box(self) -> Box<T>;
    fn into_rc(self) -> Rc<T>;
    fn into_arc(self) -> Arc<T>;
}

impl<T> Convertible<T> for T where T: Clone {
    fn into_owned(self) -> T {
        self
    }

    fn as_ref(&self) -> &T {
        &self
    }

    fn as_mut(&mut self) -> &mut T {
        &mut *self
    }

    fn into_box(self) -> Box<T> {
        Box::new(self)
    }

    fn into_rc(self) -> Rc<T> {
        Rc::new(self)
    }

    fn into_arc(self) -> Arc<T> {
        Arc::new(self)
    }
}


