struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

struct BST<T> {
    root: Option<Node<T>>
}

