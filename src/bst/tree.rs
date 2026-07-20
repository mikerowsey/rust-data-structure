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

    /// Returns the minimum element.
    pub fn min(&self) -> Option<&T> {
        let mut current = self.root.as_ref()?;

        while let Some(left) = current.left.as_ref() {
            current = left;
        }

        Some(&current.value)
    }

    /// Returns the maximum element. 
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


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> BinarySearchTree<i32> {
        let mut tree = BinarySearchTree::new();

        for value in [5, 3, 7, 1, 4, 6, 8] {
            tree.insert(value);
        }

        tree
    }

    #[test]
    fn new_tree_is_empty() {
        let tree = BinarySearchTree::<i32>::new();

        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn default_tree_is_empty() {
        let tree = BinarySearchTree::<i32>::default();

        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn insert_into_empty_tree() {
        let mut tree = BinarySearchTree::new();

        assert!(tree.insert(5));
        assert_eq!(tree.len(), 1);
        assert!(tree.contains(&5));
    }

    #[test]
    fn insert_multiple_values() {
        let tree = sample_tree();

        assert_eq!(tree.len(), 7);

        for value in [1, 3, 4, 5, 6, 7, 8] {
            assert!(tree.contains(&value));
        }
    }

    #[test]
    fn duplicate_insert_returns_false() {
        let mut tree = BinarySearchTree::new();

        assert!(tree.insert(5));
        assert!(!tree.insert(5));

        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn contains_returns_false_for_missing_value() {
        let tree = sample_tree();

        assert!(!tree.contains(&2));
        assert!(!tree.contains(&9));
    }

    #[test]
    fn contains_on_empty_tree_returns_false() {
        let tree = BinarySearchTree::<i32>::new();

        assert!(!tree.contains(&42));
    }

    #[test]
    fn min_on_empty_tree_is_none() {
        let tree = BinarySearchTree::<i32>::new();

        assert_eq!(tree.min(), None);
    }

    #[test]
    fn max_on_empty_tree_is_none() {
        let tree = BinarySearchTree::<i32>::new();

        assert_eq!(tree.max(), None);
    }

    #[test]
    fn min_returns_smallest_value() {
        let tree = sample_tree();

        assert_eq!(tree.min(), Some(&1));
    }

    #[test]
    fn max_returns_largest_value() {
        let tree = sample_tree();

        assert_eq!(tree.max(), Some(&8));
    }

    #[test]
    fn min_and_max_single_node() {
        let mut tree = BinarySearchTree::new();

        tree.insert(42);

        assert_eq!(tree.min(), Some(&42));
        assert_eq!(tree.max(), Some(&42));
    }

    #[test]
    fn clear_empties_tree() {
        let mut tree = sample_tree();

        tree.clear();

        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.min(), None);
        assert_eq!(tree.max(), None);
    }

    #[test]
    fn clear_on_empty_tree_is_safe() {
        let mut tree = BinarySearchTree::<i32>::new();

        tree.clear();

        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn inorder_returns_sorted_values() {
        let tree = sample_tree();

        let mut values = Vec::new();

        tree.inorder(|v| values.push(*v));

        assert_eq!(values, vec![1, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn height_of_empty_tree_is_zero() {
        let tree = BinarySearchTree::<i32>::new();

        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn height_of_single_node_tree_is_one() {
        let mut tree = BinarySearchTree::new();

        tree.insert(5);

        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn height_of_balanced_tree() {
        let tree = sample_tree();

        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn height_of_degenerate_tree() {
        let mut tree = BinarySearchTree::new();

        for i in 1..=7 {
            tree.insert(i);
        }

        assert_eq!(tree.height(), 7);
    }
}

