

use std::{borrow::{Borrow, BorrowMut}, cell::{Ref, RefCell, RefMut}, ops::{Deref, DerefMut}, rc::Rc};
use std::fmt::Debug;

use super::level::Level;

// Custom trait that encapsulates ownership or borrowing behavior with dereferencing capability.
pub trait OwnOrBorrow<'a, T>
where
    Self: AsRef<T> + AsMut<T> + Borrow<T> + BorrowMut<T> + Deref<Target = T> + DerefMut<Target = T> + 'a,
{
    // Additional methods can be defined here if necessary.
}

// Example implementation for a simple wrapper that owns its value.
#[derive(Debug)]
pub struct NodeHolder<T>(pub Rc<RefCell<T>>);

impl<'a, T: 'a> NodeHolder<T> {
    // Constructor for creating a new NodeHolder
    pub fn new(data: T) -> Self {
        NodeHolder(Rc::new(RefCell::new(data)))
    }

    // Getter that provides an immutable reference to T
    pub fn get(&self) -> Ref<'_, T> {
        (*self.0).borrow()
    }

    // Method that provides a mutable reference to T
    pub fn get_mut(&self) -> RefMut<'_, T> {
        (*self.0).borrow_mut()
    }

    // Setter that allows modifying T
    // pub fn set(&self, value: T) {
    //     let mut temp = (self.0).borrow_mut();
    //     temp = value;
    // }

    pub fn find(&self, price: &u64) -> Option<NodeHolder<LevelNode>> {
        todo!()
    }

    // Method to clone the Rc<RefCell<T>> for shared ownership
    pub fn clone_inner(&self) -> Rc<RefCell<T>> {
        Rc::clone(&self.0)
    }
}

pub trait TreeOps {
    fn find_node_by_price(&mut self, price: u64) -> Option<NodeHolder<LevelNode>>;
    fn insert(&mut self, candidate_node: Option<NodeHolder<LevelNode>>);
    fn get_next_lower_level(&mut self, level_node: NodeHolder<LevelNode>) -> Option<NodeHolder<LevelNode>>;
    fn get_next_higher_level(&mut self, level_node: NodeHolder<LevelNode>) -> Option<NodeHolder<LevelNode>>;
}

impl TreeOps for NodeHolder<LevelNode> {

    fn find_node_by_price(&mut self, price: u64) -> Option<NodeHolder<LevelNode>>                                           
    {
        let mut current = Some(*self);
        while let Some(node) = current {
            let borrowed_node = *node.get();
            let node_price = borrowed_node.level.price;
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
    
    fn insert(&mut self, candidate_node: Option<NodeHolder<LevelNode>>)                                           
    {
        let mut new_node = candidate_node.expect("new node insertion").get_mut();
        let mut this_node = self.get_mut();
        if (*new_node).level.price < (*this_node).level.price {
            if let Some(mut left) = this_node.left {
                left.insert(candidate_node);
            } else {
                // replace with this
                new_node.parent = Some(self.clone());
                this_node.left = candidate_node.clone();
            }
        } else {
            if let Some(mut right) = this_node.right {
                right.insert(candidate_node);
            } else {
                // replace with this
                new_node.parent = Some(self.clone());
                this_node.right = candidate_node.clone();
            }
        }
    }

    fn get_next_lower_level(&mut self, level_node: NodeHolder<LevelNode>) -> Option<NodeHolder<LevelNode>>                                         
    {
        if let Some(left_child) = level_node.get().left {
            // If there is a left child, go left and then as far right as possible
            let mut current = left_child;
            while let Some(right_child) = current.get().right {
                current = right_child;
            }
        } else {
            // If there is no left child, go up until you find a smaller node
            while let Some(parent) = level_node.get().parent {
                if parent.get().level.price < self.get().level.price {
                    return Some(parent);
                }
                *self = parent;
            }
        }
        Some(*self)
    }

    fn get_next_higher_level(&mut self, level_node: NodeHolder<LevelNode>) -> Option<NodeHolder<LevelNode>>                                             
    {
        if let Some(right_child) = self.get().right {
            // If there is a right child, go right and then as far left as possible
            *self = right_child;
            while let Some(left_child) = self.get().left {
                *self = left_child.clone();
            }
        } else {
            // If there is no right child, go up until you find a greater node
            while let Some(parent) = self.get().parent {
                if parent.get().level.price > self.get().level.price {
                    return Some(parent);
                }
                *self = parent.clone();
            }
        }
        Some(self.clone())
    }
}

pub trait TreeRemoval {
    fn remove(&mut self, price: u64) -> Option<NodeHolder<LevelNode>>;
    fn remove_recursive(&mut self, price: u64) -> Option<NodeHolder<LevelNode>>;
    fn find_min(&mut self) -> NodeHolder<LevelNode>;
}

impl TreeRemoval for NodeHolder<LevelNode>  {
    
    fn remove(&mut self, price: u64) -> Option<NodeHolder<LevelNode>> 
    {
        // This is a placeholder implementation and may need adjustments
        // based on the specific requirements of your binary tree.
        // For example, you might need to handle rebalancing the tree
        // after removal, which is not covered here.

        // Note: This implementation assumes `self.root` exists as part of the LevelNode structure.
        self.remove_recursive(price)
    }

    fn remove_recursive(&mut self, price: u64) -> Option<NodeHolder<LevelNode>> {
        let current = Some(self);
        if let Some(node) = current {
            let mut node_borrowed = node.get_mut();
            if price < node_borrowed.level.price {
                node_borrowed.left = node_borrowed.left.expect("left node not retrieved").remove_recursive(price);
            } else if price > node_borrowed.level.price {
                node_borrowed.right = node_borrowed.right.expect("right node not retrieved").remove_recursive(price);
            } else {
                if node_borrowed.left.is_none() {
                    return node_borrowed.right.take();
                } else if node_borrowed.right.is_none() {
                    return node_borrowed.left.take();
                }

                let successor_price = node_borrowed.right.expect("right node").find_min().get().level.price;
                node_borrowed.level.price = successor_price;
                node_borrowed.right = node_borrowed.right.expect("right node not retrieved").remove_recursive(successor_price);
            }
            None
        } else {
            None
        }
    }
    
    // Helper function to find the minimum node starting from a given node
    fn find_min(&mut self) -> NodeHolder<LevelNode>                                    
    {
        let mut current = *self;
        while let Some(left) = current.get().left {
            current = left;
        }
        current
    }
}

impl<T> PartialEq for NodeHolder<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// Implementing Deto return Ref<T>, allowing access to T's methods via the dot operator.
impl<T> Deref for NodeHolder<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implementing the Clone trait for NodeHolder
impl<T> Clone for NodeHolder<T> {
    fn clone(&self) -> Self {
        NodeHolder(Rc::clone(&self.0))
    }
}

#[derive(Debug)]
pub struct LevelNode
{
    pub level: Level, // Assuming Level is an u64 for simplicity
    pub parent: Option<NodeHolder<LevelNode>>,
    pub left: Option<NodeHolder<LevelNode>>,
    pub right: Option<NodeHolder<LevelNode>>,
}

// Helper function to find the minimum node starting from a given node
fn find_min(node: NodeHolder<LevelNode>) -> NodeHolder<LevelNode>                                    
{
    let mut current = node;
    while let Some(left) = current.get().left {
        current = left;
    }
    current
}


// struct InOrderIterator 
// where
//     
// {
//     stack: Vec,
//     next_node: Option<NodeHolder<LevelNode>>,
//     _marker: PhantomData<(&T, PhantomData)>,
// }

// // impl<'a> LevelNode {
// //     fn in_order_iterator(&self) -> InOrderIterator<'a> {
// //         let mut iterator = InOrderIterator {
// //             stack: Veself.new(),
// //             next_node: self,
// //         };
// //         iterator.move_to_leftmost();
// //         iterator
// //     }
// // }

// impl InOrderIterator
// where
//     
// {
//     fn move_to_leftmost(&mut self) {
//         while let Some(node) = self.next_node.get() {
//             self.stack.push(node.get());
//             self.next_node = node.get().left;
//         }
//     }
// }

// impl Iterator for InOrderIterator
// where
//       // Assuming T is a type that implements TreeOps. This might be redundant based on your actual trait definitions.
// {
//     type Item = T; // Assuming A is defined elsewhere

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(node) = self.stack.pop() {
//             self.next_node = node.get().right;
//             self.move_to_leftmost();
//             Some(node) // This line may need adjustment based on the actual return type you intended for `Some(node)`
//         } else {
//             None
//         }
//     }
// }
