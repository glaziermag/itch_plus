use std::{cell::{RefCell, RefMut}, rc::Rc, sync::{Mutex, Arc, MutexGuard}, marker::PhantomData};

//use crate::references::LevelMut;

use crate::order_book::order_book::{Mutable, OrderBook};

use super::level::Level;

pub trait Ref<'a>: AccessContents<MutateContents> + 'a {}

pub trait MutableBook<'a>: Mutable<OrderBook<Ref<'a>>> {}

pub trait AccessContents<T> {
    type Target;
    fn access(&self) -> Self::Target;
}

// For Rc<RefCell<T>>irectly access the inner RefCell
impl<T> AccessContents<T> for Rc<T> {
    type Target = T;

    fn access(&self) -> Self::Target {
        *self
    }
}

// For Arc<Mutex<T>>irectly access the inner Mutex
impl<T> AccessContents<T> for Arc<T> {
    type Target = T;

    fn access(&self) -> Self::Target {
        *self
    }
}
pub trait MutateContents {
    type Target;
    // Return a type that allows further operations.
    fn mutate(&self) -> Self::Target;
}

// For RefCell<T>, return a RefMut<T>
impl<T> MutateContents for RefCell<T> {
    type Target<'a> = RefMut<'a, T> where T: 'a;

    fn mutate(&self) -> Self::Target {
        self.borrow_mut()
    }
}

// For Mutex<T>, return a MutexGuard<T>
impl<T> MutateContents for Mutex<T> {
    type Target<'a> = MutexGuard<'a, T> where T: 'a;

    fn mutate(&self) -> Self::Target {
        self.lock().expect("Mutex lock failed")
    }
}

#[derive()]
pub struct LevelNode<'a, R>
where
    R: Ref<'a>,
{
    // LevelMut<'a>
    pub level: Level<'a, R>, // Assuming Level is an u64 for simplicity
    pub parent: Option<R>,
    pub left: Option<R>,
    pub right: Option<R>,
    pub(crate) _marker: PhantomData<&'a M>,
}

pub trait Tree<'a, R> 
where
    R: Ref<'a>,
{
    fn insert(this_node: Option<R>, new_node: Option<R>) where Self: Sized;
    fn get_next_level_node(level_node: R) -> Option<R>;
    fn get_next_lower_level(level_node: R) -> Option<R>;
    fn get_next_higher_level(level_node: R) -> Option<R>;
    fn get(node: Option<R>, price: u64) -> Option<R>;
    fn remove(node: Option<R>, price: u64) -> Option<R>;
    fn remove_recursive(node: Option<R>, price: u64) -> Option<R>;
    fn find_min(node: A) -> A;
}

// Helper function to find the minimum node starting from a given node
fn find_min<'a, B, T>(node: T) -> T 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>, 
{
    let mut current = node;
    while let Some(left) = current.borrow().left.clone() {
        current = left;
    }
    current
}

fn get<'a, T,  B>(node: Option<R>, price: u64) -> Option<R> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>, 
{
    let mut current = node;
    while let Some(node) = current {
        let borrowed_node = node.borrow();
        let node_price = borrowed_node.price;
        if price < node_price {
            current = borrowed_node.left;
        } else if price > node_price {
            current = borrowed_node.right;
        } else {
            return current;
        }
    }
    None
}

fn remove<'a, B, R>(mut node: Option<R>, price: u64) -> Option<R> 
where
    R: Ref<'a>,
    T: Tree<'a, R> 
{
    // This is a placeholder implementation and may need adjustments
    // based on the specific requirements of your binary tree.
    // For example, you might need to handle rebalancing the tree
    // after removal, which is not covered here.

    // Note: This implementation assumes `self.root` exists as part of the LevelNode structure.
    node = T::remove_recursive(node.take(), price);
    node.clone()
}

fn remove_recursive<'a, T, B>(node: Option<R>, price: u64) -> Option<R> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>, 
{
    let node = match node {
        Some(n) => n,
        None => return None,
    };

    if price < node.borrow().price {
        let left = T::remove_recursive(node.borrow().left.clone(), price);
        node.borrow_mut().left = left;
    } else if price > node.borrow().price {
        let right = T::remove_recursive(node.borrow().right.clone(), price);
        node.borrow_mut().right = right;
    } else {
        // Node with only one child or no child
        if node.borrow().left.is_none() {
            return node.borrow().right.clone();
        } else if node.borrow().right.is_none() {
            return node.borrow().left.clone();
        }

        // Node with two children: Get the inorder successor (smallest in the right subtree)
        let temp = T::find_min(node.borrow().right.clone().unwrap());
        node.borrow_mut().price = temp.borrow().price;
        node.borrow_mut().right = T::remove_recursive(node.borrow().right.clone(), temp.borrow().price);
    }

    Some(node)
}

fn insert<'a, T, B>(this_node: Option<R>, new_node: Option<R>) 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>, 
{

    if new_node.borrow().price < this_node.borrow().price {
        if let Some(left) = this_node.borrow().left {
            T::insert(left, new_node);
        } else {
            new_node.borrow_mut().parent = Some(Rc::clone(&this_node));
            this_node.borrow_mut().left = Some(new_node);
        }
    } else {
        if let Some(right) = this_node.borrow().right {
            T::insert(right, new_node);
        } else {
            new_node.borrow_mut().parent = Some(Rc::clone(&this_node));
            this_node.borrow_mut().right = Some(new_node);
        }
    }
}

fn get_next_level_node<'a, T, B>(level_node: R) -> Option<R> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>, 
{
    if (*level_node).borrow().level.is_bid() {
        // For a bid, find the next lower level
        T::get_next_lower_level(level_node)
    } else {
        // For an ask, find the next higher level
        T::get_next_higher_level(level_node)
    }
}

fn get_next_lower_level<'a, T, B>(mut level_node: R) -> Option<R> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>, 
{
    
    if let Some(left_child) = level_node.borrow().left {
        // If there is a left child, go left and then as far right as possible
        let mut current = left_child;
        while let Some(right_child) = current.borrow().right {
            current = right_child;
        }
    } else {
        // If there is no left child, go up until you find a smaller node
        while let Some(parent) = level_node.borrow().parent {
            if parent.borrow().price < level_node.borrow().price {
                return Some(parent);
            }
            level_node = parent;
        }
    }
    Some(level_node)
}

fn get_next_higher_level<'a, T, B>(mut level_node: R) -> Option<R> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>, 
{

    if let Some(right_child) = level_node.borrow().right {
        // If there is a right child, go right and then as far left as possible
        level_node = right_child;
        while let Some(left_child) = level_node.borrow().left {
            level_node = left_child;
        }
    } else {
        // If there is no right child, go up until you find a greater node
        while let Some(parent) = level_node.borrow().parent {
            if parent.borrow().price > level_node.borrow().price {
                return Some(parent);
            }
            level_node = parent;
        }
    }
    Some(level_node)
}



struct InOrderIterator<'a, R> 
where
    R: Ref<'a>,
    T: Tree<'a, R>,
    B: MutableBook<'a>,
{
    stack: Vec<T>,
    next_node: Option<R>,
    _marker: PhantomData<(&'a D, PhantomData<M>)>,
}

// impl<'a> LevelNode<'a, R> {
//     fn in_order_iterator(&self) -> InOrderIterator<'a> {
//         let mut iterator = InOrderIterator {
//             stack: VeB::new(),
//             next_node: self.clone(),
//         };
//         iterator.move_to_leftmost();
//         iterator
//     }
// }

impl<'a, B: MutableBook<'a, M, R> InOrderIterator<'a, R> {
    fn move_to_leftmost(&mut self) {
        while let Some(node) = self.next_node.clone() {
            self.stack.push(node.clone());
            self.next_node = node.borrow().left.clone();
        }
    }
}

impl<'a, B: MutableBook<'a>, R: Ref<'a>, T: Tree<'a, R>> Iterator for InOrderIterator<'a, R> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            self.next_node = node.borrow().right.clone();
            self.move_to_leftmost();
            Some(node)
        } else {
            None
        }
    }
}
