use rust_data_structures::bst::BinarySearchTree;

#[test]
fn new_tree_is_empty() {
    let tree = BinarySearchTree::<i32>::new();

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
}

#[test]
fn insert_and_contains() {
    let mut tree = BinarySearchTree::<i32>::new();

    tree.insert(10);

    assert!(tree.contains(&10));
}
