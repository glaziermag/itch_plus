use std::{cell::RefCell, rc::Rc, ops::Deref};

use super::level::Level;

pub type RcCell<T> = Rc<RefCell<T>>;

pub type RcNode<'a> = RcCell<LevelNode<'a>>;

pub struct LevelNode<'a> {
    pub level: Level<'a>, // Assuming Level is an i64 for simplicity
    pub parent: Option<RcNode<'a>>,
    pub left: Option<RcNode<'a>>,
    pub right: Option<RcNode<'a>>,
}

impl<'a> LevelNode<'a> {
    // Helper function to find the minimum node starting from a given node
    fn find_min(node: RcNode<'a>) -> RcNode<'a> {
        let mut current = node;
        while let Some(left) = current.borrow().left.clone() {
            current = left;
        }
        current
    }
}

impl<'a> Deref for LevelNode<'a> {
    type Target = Level<'a>;

    fn deref(&self) -> &Self::Target {
        &self.level
    }
}

pub trait AsDeref<'a> {
    fn as_deref(&self) -> std::cell::Ref<'_, LevelNode<'a>>;
}

impl<'a> AsDeref<'a> for RcNode<'a> {
    fn as_deref(&self) -> std::cell::Ref<'_, LevelNode<'a>> {
        self.borrow()
    }
}

pub trait Tree<'a> {
    fn insert(this_node: RcNode, new_node: RcNode) where Self: Sized;
    fn get_next_level_node(level_node: RcNode) -> Option<RcNode<'a>>;
    fn get_next_lower_level(level_node: RcNode) -> Option<RcNode<'a>>;
    fn get_next_higher_level(level_node: RcNode) -> Option<RcNode<'a>>;
    fn get(&self, price: u64) -> Option<RcNode<'a>>;
    fn remove(&mut self, price: u64) -> Option<RcNode<'a>>;
    fn remove_recursive(node: Option<RcNode<'a>>, price: u64) -> Option<RcNode<'a>>;
}


impl<'a> Tree<'a> for LevelNode<'_> {

    fn get(&self, price: u64) -> Option<RcNode<'a>> {
        let mut current = Some(Rc::clone(&Rc::new(RefCell::new(*self))));
        while let Some(node) = current {
            let borrowed_node = node.borrow();
            let node_price = borrowed_node.price;
            if price < node_price {
                current = borrowed_node.left;
            } else if price > node_price {
                current = borrowed_node.right;
            } else {
                return Some(node);
            }
        }
        None
    }

    fn remove(&mut self, price: u64) -> Option<RcNode<'a>> {
        // This is a placeholder implementation and may need adjustments
        // based on the specific requirements of your binary tree.
        // For example, you might need to handle rebalancing the tree
        // after removal, which is not covered here.

        // Note: This implementation assumes `self.root` exists as part of the LevelNode structure.
        self.root = Self::remove_recursive(self.root.take(), price);
        self.root.clone()
    }

    fn remove_recursive(node: Option<RcNode<'a>>, price: u64) -> Option<RcNode<'a>> {
        let node = match node {
            Some(n) => n,
            None => return None,
        };

        if price < node.borrow().price {
            let left = Self::remove_recursive(node.borrow().left.clone(), price);
            node.borrow_mut().left = left;
        } else if price > node.borrow().price {
            let right = Self::remove_recursive(node.borrow().right.clone(), price);
            node.borrow_mut().right = right;
        } else {
            // Node with only one child or no child
            if node.borrow().left.is_none() {
                return node.borrow().right.clone();
            } else if node.borrow().right.is_none() {
                return node.borrow().left.clone();
            }

            // Node with two children: Get the inorder successor (smallest in the right subtree)
            let temp = Self::find_min(node.borrow().right.clone().unwrap());
            node.borrow_mut().price = temp.borrow().price;
            node.borrow_mut().right = Self::remove_recursive(node.borrow().right.clone(), temp.borrow().price);
        }

        Some(node)
    }

    fn insert(this_node: RcNode, new_node: RcNode) {

        if new_node.borrow().price < this_node.borrow().price {
            if let Some(left) = this_node.borrow().left {
                Self::insert(left, new_node);
            } else {
                new_node.borrow_mut().parent = Some(Rc::clone(&this_node));
                this_node.borrow_mut().left = Some(new_node);
            }
        } else {
            if let Some(right) = this_node.borrow().right {
                Self::insert(right, new_node);
            } else {
                new_node.borrow_mut().parent = Some(Rc::clone(&this_node));
                this_node.borrow_mut().right = Some(new_node);
            }
        }
    }

    fn get_next_level_node(level_node: RcNode) -> Option<RcNode<'a>> {
        if (*level_node.borrow()).level.is_bid() {
            // For a bid, find the next lower level
            Self::get_next_lower_level(level_node)
        } else {
            // For an ask, find the next higher level
            Self::get_next_higher_level(level_node)
        }
    }

    fn get_next_lower_level(mut level_node: RcNode) -> Option<RcNode<'a>> {
        
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

    fn get_next_higher_level(mut level_node: RcNode) -> Option<RcNode<'a>> {

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
}


struct InOrderIterator<'a> {
    stack: Vec<RcNode<'a>>,
    next_node: Option<RcNode<'a>>,
}

// impl<'a> LevelNode<'a> {
//     fn in_order_iterator(&self) -> InOrderIterator<'a> {
//         let mut iterator = InOrderIterator {
//             stack: Vec::new(),
//             next_node: self.clone(),
//         };
//         iterator.move_to_leftmost();
//         iterator
//     }
// }

impl<'a> InOrderIterator<'a> {
    fn move_to_leftmost(&mut self) {
        while let Some(node) = self.next_node.clone() {
            self.stack.push(node.clone());
            self.next_node = node.borrow().left.clone();
        }
    }
}

impl<'a> Iterator for InOrderIterator<'a> {
    type Item = RcNode<'a>;

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
