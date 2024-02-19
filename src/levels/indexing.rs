

use std::{cell::{RefCell}, rc::{Rc, Weak}};
use std::fmt::Debug;

use crate::orders::order::ErrorCode;

use super::level::Level;

pub trait TreeOps {
    fn find_node_by_price(&mut self, price: u64) -> Option<Rc<RefCell<LevelNode>>>;
    fn insert(&mut self, candidate_node: Rc<RefCell<LevelNode>>);
    fn get_next_lower_level(&self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode>;
    fn get_next_higher_level(&self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode>;
}

impl TreeOps for Rc<RefCell<LevelNode>> {

    fn find_node_by_price(&mut self, price: u64) -> Option<Rc<RefCell<LevelNode>>>                                          
    {
        let mut current = Some(*self);
        while let Some(node) = current {
            if let Ok(borrowed_node) = node.try_borrow() {
                let node_price = borrowed_node.level.price;
                if price < node_price {
                    current = borrowed_node.left.clone();
                } else if price > node_price {
                    current = borrowed_node.right.clone();
                } else {
                    return Some(node.clone());
                }
            } else {
                // Optionally handle the error or simply skip/break
                break;
            }
        }
        None
    }
    
    fn insert(&mut self, candidate_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode>                        
    {
        let candidate_price = candidate_node.try_borrow().map(|n| n.level.price).expect("Failed to borrow candidate node");
        let mut current = Some(self);
        loop {
            match current {
                Some(node) => {
                    let mut borrowed_node = node.try_borrow_mut().expect("Failed to borrow current node");
                    if candidate_price < borrowed_node.level.price {
                        if let Some(left) = &mut borrowed_node.left {
                            current = Some(left);
                        } else {
                            borrowed_node.left = Some(candidate_node.clone()); // Insert here
                            // Assuming `set_parent` properly sets the parent of the node.
                            candidate_node.set_parent(node);
                            break;
                        }
                    } else {
                        if let Some(right) = &mut borrowed_node.right {
                            current = Some(right);
                        } else {
                            borrowed_node.right = Some(candidate_node.clone()); // Insert here
                            // Assuming `set_parent` properly sets the parent of the node.
                            candidate_node.set_parent(node);
                            break;
                        }
                    }
                },
                None => break,
            }
        }
    }

    fn get_next_lower_level(&self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        // verify rc clone borrow here
        let mut current: Option<&Rc<RefCell<LevelNode>>> = Some(&level_node);
        let mut last_smaller: Option<Rc<RefCell<LevelNode>>> = None;
        
        loop {
            match current {
                Some(node) => {
                    let borrowed_node = node.try_borrow().expect("Failed to borrow node");
                    
                    if let Some(left) = &borrowed_node.left {
                        // Go left once
                        let mut deepest_right = left;
                        // Then go right as far as possible
                        while let Some(right) = deepest_right.try_borrow().expect("should borrow right").right.as_ref() {
                            deepest_right = right;
                        }
                        return Some(deepest_right.clone());
                    } else {
                        // No left child, traverse up to find a smaller node
                        current = borrowed_node.parent.as_ref().map(|p| p.upgrade().expect("Failed to upgrade weak reference"));
                        if current.is_some() && current.unwrap().try_borrow().expect("Failed to borrow parent").level.price < borrowed_node.level.price {
                            last_smaller = current.cloned();
                        } else {
                            break;
                        }
                    }
                },
                None => break,
            }
        }
        
        last_smaller
    }

    fn get_next_higher_level(&self, level_node: Rc<RefCell<LevelNode>>) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        let mut current: Option<&Rc<RefCell<LevelNode>>> = Some(&level_node);
        let mut last_greater: Option<Rc<RefCell<LevelNode>>> = None;
        
        loop {
            match current {
                Some(node) => {
                    let borrowed_node = node.try_borrow().expect("Failed to borrow node");
                    
                    if let Some(right) = &borrowed_node.right {
                        // Go right once
                        let mut deepest_left = right;
                        // Then go left as far as possible
                        while let Some(left) = deepest_left.try_borrow().expect("should borrow left").left.as_ref() {
                            deepest_left = left;
                        }
                        return Ok(Some(deepest_left.clone()));
                    } else {
                        // No right child, traverse up to find a greater node
                        current = borrowed_node.parent.as_ref().map(|p| p.upgrade().as_ref().expect("Failed to upgrade weak reference"));
                        if current.is_some() && current.unwrap().try_borrow().expect("Failed to borrow parent").level.price > borrowed_node.level.price {
                            last_greater = current.cloned();
                        } else {
                            break;
                        }
                    }
                },
                None => break,
            }
        }
        
        last_greater
    }
}

pub trait TreeRemoval {
    fn remove(&mut self, price: u64) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode>;
    fn remove_recursive(&mut self, price: u64) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode>;
    fn find_min(&self) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode>;
}

impl TreeRemoval for Rc<RefCell<LevelNode>>  {
    
    fn remove(&mut self, price: u64) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode>
    {
        self.remove_recursive(price)
    }

    fn remove_recursive(&mut self, price: u64) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        let mut node_borrowed = self.try_borrow_mut().map_err(|_| ErrorCode::OtherError("Failed to borrow node for modification".to_string()))?;
        if price < node_borrowed.level.price {
            if let Some(left) = &node_borrowed.left {
                node_borrowed.left = left.remove_recursive(price);
            } else {
                return Err(ErrorCode::OrderNotFound);
            }
        } else if price > node_borrowed.level.price {
            if let Some(right) = &node_borrowed.right {
                node_borrowed.right = right.remove_recursive(price);
            } else {
                return Err(ErrorCode::OrderNotFound);
            }
        } else {
            if node_borrowed.left.is_none() {
                return Ok(node_borrowed.right.take());
            } else if node_borrowed.right.is_none() {
                return Ok(node_borrowed.left.take());
            }

            let successor = node_borrowed.right.as_ref().ok_or(ErrorCode::OrderNotFound)?.find_min();
            let successor_price = successor.try_borrow().map_err(|_| ErrorCode::OtherError("Failed to borrow successor for price retrieval".to_string()))?.level.price;
            node_borrowed.level.price = successor_price;
            node_borrowed.right = node_borrowed.right.as_ref().ok_or(ErrorCode::OrderNotFound)?.remove_recursive(successor_price);
        }
        Ok(None)
    }
    
    // Helper function to find the minimum node starting from a given node
    fn find_min(&self) -> Result<Option<Rc<RefCell<LevelNode>>>, ErrorCode> {
        let mut current = self.clone();
        loop {
            let borrowed_current = current.try_borrow().map_err(|_| ErrorCode::OtherError("Failed to borrow node for minimum search".to_string()))?;
            match &borrowed_current.left {
                Some(left) => current = left.clone(),
                None => break,
            }
        }
        Ok(current)
    }
}

#[derive(Debug)]
pub struct LevelNode
{
    pub level: Level,
    pub parent: Option<Weak<RefCell<LevelNode>>>,
    pub left: Option<Rc<RefCell<LevelNode>>>,
    pub right: Option<Rc<RefCell<LevelNode>>>,
}

impl LevelNode {
    // Function that returns the level of the node.
    // For demonstration, we use Result to encapsulate the level or an error.
    pub fn level(&self) -> Result<&Level, ErrorCode> {
        Ok(&self.level)
    }

    pub fn level_mut(&mut self) -> Result<&mut Level, ErrorCode> {
        Ok(&mut self.level)
    }
}


// struct InOrderIterator 
// where
//     
// {
//     stack: Vec,
//     next_node: Option<Rc<RefCell<LevelNode>>>,
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
//         while let Some(node) = self.next_node.try_borrow() {
//             self.stack.push(node.try_borrow());
//             self.next_node = node.try_borrow().left;
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
//             self.next_node = node.try_borrow().right;
//             self.move_to_leftmost();
//             Some(node) // This line may need adjustment based on the actual return type you intended for `Some(node)`
//         } else {
//             None
//         }
//     }
// }
