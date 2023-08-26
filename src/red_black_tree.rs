use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct RedBlackTree<T> {
    root: Link<T>,
}

impl<T: PartialOrd + PartialEq + Eq> RedBlackTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn push(&mut self, new_value: T) {}
}

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    value: T,
    color: Color,
    parent: Link<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T: PartialOrd + PartialEq + Eq> Node<T> {
    fn new_link(value: T, color: Color, parent: Link<T>, left: Link<T>, right: Link<T>) -> Link<T> {
        Some(Rc::new(RefCell::new(Node {
            value: value,
            color: color,
            parent: parent,
            left: left,
            right: right,
        })))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_for_bst() {
        let mut tree = RedBlackTree::<i8>::new();

        // check initialization
        assert_eq!(RedBlackTree { root: None }, tree);

        tree.push(1);
        assert_eq!(
            RedBlackTree {
                root: Node::<i8>::new_link(1, Color::Black, None, None, None)
            },
            tree
        )
    }
}
