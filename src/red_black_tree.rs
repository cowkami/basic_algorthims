type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
    color: Color,
}

#[derive(Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct RedBlackTree<T> {
    root: Link<T>,
}
