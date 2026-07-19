pub(crate) type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub(crate) struct Node<T> {
    pub(crate) value: T,
    pub(crate) left: Link<T>,
    pub(crate) right: Link<T>,
}

impl<T> Node<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_has_no_children() {
        let node = Node::new(42);

        assert_eq!(node.value, 42);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }
}
