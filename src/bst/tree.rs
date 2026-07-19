#![allow(dead_code)]

use super::node::{Link, Node};
use std::cmp::Ordering;
use std::fmt::Display;

/// A binary search tree storing unique values in sorted order.
///
/// Values are ordered according to their `Ord` implementation.
/// Duplicate values are ignored.
///
/// Insert, lookup, and removal operate in **O(h)** time, where `h`
/// is the height of the tree. In the worst case an unbalanced tree
/// has height `n`.
#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Link<T>,
    len: usize,
}

//
// Public API
//

impl<T> BinarySearchTree<T> {
    /// Creates an empty binary search tree.
    #[must_use]
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns the height of the tree.
    ///
    /// An empty tree has height `0`.
    #[must_use]
    pub fn height(&self) -> usize {
        Self::height_node(&self.root)
    }

    /// Traverses the tree in sorted (inorder) order.
    ///
    /// The supplied closure is called once for every value in the tree.
    pub fn inorder<F>(&self, mut visit: F)
    where
        F: FnMut(&T),
    {
        Self::inorder_node(&self.root, &mut visit);
    }

    /// Returns the number of elements in the tree.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the tree contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all elements from the tree.
    ///
    /// This operation runs in O(n) time because every node must be dropped.
    /// After calling `clear()`, the tree is empty.
    pub fn clear(&mut self) {
        self.root = None;
        self.len = 0;
    }

    pub fn min(&self) -> Option<&T> {
        let mut current = self.root.as_ref()?;

        while let Some(left) = current.left.as_ref() {
            current = left;
        }

        Some(&current.value)
    }

    pub fn max(&self) -> Option<&T> {
        let mut current = self.root.as_ref()?;

        while let Some(right) = current.right.as_ref() {
            current = right;
        }

        Some(&current.value)
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// Inserts a value into the tree.
    ///
    /// Returns `true` if the value was inserted.
    /// Returns `false` if the value already exists.
    pub fn insert(&mut self, value: T) -> bool {
        let success = Self::insert_node(&mut self.root, value);

        if success {
            self.len += 1;
        }

        success
    }

    /// Returns `true` if the tree contains the specified value.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        Self::contains_node(&self.root, value)
    }
}

impl<T: Display> BinarySearchTree<T> {
    /// Prints the tree contents in sorted order.
    pub fn print_inorder(&self) {
        self.inorder(|v| print!("{v} "));
        println!();
    }
}

impl<T> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

//
// Private implementation
//

impl<T> BinarySearchTree<T> {
    fn height_node(root: &Link<T>) -> usize {
        match root {
            None => 0,

            Some(node) => 1 + Self::height_node(&node.left).max(Self::height_node(&node.right)),
        }
    }

    fn inorder_node<F>(root: &Link<T>, visit: &mut F)
    where
        F: FnMut(&T),
    {
        if let Some(node) = root {
            Self::inorder_node(&node.left, visit);
            visit(&node.value);
            Self::inorder_node(&node.right, visit);
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    fn insert_node(root: &mut Link<T>, value: T) -> bool {
        match root {
            None => {
                *root = Some(Box::new(Node::new(value)));
                true
            }

            Some(node) => {
                let child = match value.cmp(&node.value) {
                    Ordering::Less => &mut node.left,
                    Ordering::Greater => &mut node.right,
                    Ordering::Equal => return false,
                };

                Self::insert_node(child, value)
            }
        }
    }

    fn contains_node(root: &Link<T>, value: &T) -> bool {
        match root {
            None => false,

            Some(node) => {
                let child = match value.cmp(&node.value) {
                    Ordering::Less => &node.left,
                    Ordering::Greater => &node.right,
                    Ordering::Equal => return true,
                };

                Self::contains_node(child, value)
            }
        }
    }
}
