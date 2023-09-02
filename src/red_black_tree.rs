use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct RedBlackTree<T> {
    root: Link<T>,
}

impl<T: PartialOrd + PartialEq + Eq + Copy> RedBlackTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn in_order_traverse(&self) -> Vec<T> {
        Node::in_order_traverse(&self.root)
    }

    fn push(&mut self, new_key: T) {
        if let Some(node) = self.root.take() {
            node.borrow_mut().push(new_key);
        } else {
            self.root = Node::new_link(new_key, Color::Black, None, None, None)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    key: T,
    color: Color,
    parent: Link<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T: PartialOrd + PartialEq + Eq + Copy> Node<T> {
    fn new_link(key: T, color: Color, parent: Link<T>, left: Link<T>, right: Link<T>) -> Link<T> {
        Some(Rc::new(RefCell::new(Node {
            key: key,
            color: color,
            parent: parent,
            left: left,
            right: right,
        })))
    }

    fn push(&mut self, new_key: T) {}

    fn in_order_traverse(link: &Link<T>) -> Vec<T> {
        match link.as_ref() {
            Some(node) => [
                Node::in_order_traverse(&node.borrow().left),
                vec![node.borrow().key],
                Node::in_order_traverse(&node.borrow().right),
            ]
            .concat(),
            None => vec![],
        }
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
    fn push_for_rbt() {
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

    #[test]
    fn in_order_traverse() {
        let tree = RedBlackTree {
            root: Node::<i8>::new_link(
                1,
                Color::Black,
                None,
                Node::<i8>::new_link(
                    2,
                    Color::Black,
                    None,
                    Node::<i8>::new_link(3, Color::Black, None, None, None),
                    Node::<i8>::new_link(4, Color::Black, None, None, None),
                ),
                Node::<i8>::new_link(5, Color::Black, None, None, None),
            ),
        };
        assert_eq!(vec![3, 2, 4, 1, 5], tree.in_order_traverse());
    }
}
