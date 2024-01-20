use std::cell::RefCell;
use std::rc::{Rc, Weak};


pub trait Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> where Self: Sized;
    fn insert_left(&self, child: Rc<RefCell<Self>>) where Self: Sized;
    fn insert_right(&self, child: Rc<RefCell<Self>>) where Self: Sized;
    fn left(&self) -> Option<Rc<RefCell<Self>>> where Self: Sized;
    fn right(&self) -> Option<Rc<RefCell<Self>>> where Self: Sized;
    fn parent(&self) -> Option<Weak<RefCell<Self>>> where Self: Sized;
}

pub type TreeNode = LinkNode<u64>;

pub struct LinkNode<T> {
    price: T,
    parent: Option<Weak<RefCell<LinkNode<T>>>>,
    left: Option<Rc<RefCell<LinkNode<T>>>>,
    right: Option<Rc<RefCell<LinkNode<T>>>>,
}

impl<T>Node<T> for LinkNode<T> {
    
    fn new(price: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(LinkNode {
            price,
            parent: None,
            left: None,
            right: None,
        }))
    }

    fn insert_left(&self, child: Rc<RefCell<Self>>) {
        self.left.replace(child.clone());
        child.borrow_mut().parent = Some(Rc::downgrade(&Rc::new(RefCell::new(*self))));
    }

    fn insert_right(&self, child: Rc<RefCell<Self>>) {
        self.right.replace(child.clone());
        child.borrow_mut().parent = Some(Rc::downgrade(&Rc::new(RefCell::new(*self))));
    }

    fn left(&self) -> Option<Rc<RefCell<Self>>> {
        self.left.clone()
    }

    fn right(&self) -> Option<Rc<RefCell<Self>>> {
        self.right.clone()
    }

    fn parent(&self) -> Option<Weak<RefCell<Self>>> {
        self.parent.clone()
    }
}