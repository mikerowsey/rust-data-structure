pub(crate) type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub(crate) struct Node<T> {
    pub(crate) value: T,
    pub(crate) left: Option<Box<Node<T>>>,
    pub(crate) right: Option<Box<Node<T>>>,
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
