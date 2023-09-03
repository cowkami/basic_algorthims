use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct RedBlackTree<T> {
    root: Link<T>,
}

impl<T: PartialEq + Eq> PartialEq for RedBlackTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<T: PartialOrd + PartialEq + Eq + Copy> RedBlackTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn inorder_traverse(&self) -> Vec<T> {
        Node::inorder_traverse(&self.root)
    }

    fn rotate(&self, node: &Link<T>, direction: Direction) {}

    fn push(&mut self, key: T) {
        if let Some(node) = self.root.take() {
            node.borrow_mut().push(key);
        } else {
            self.root = Node::new_link(key, Color::Black, None, None, None)
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    key: T,
    color: Color,
    parent: WeakLink<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T: PartialEq + Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.color == other.color
    }
}

impl<T: PartialOrd + PartialEq + Eq + Copy> Node<T> {
    fn new_link(
        key: T,
        color: Color,
        parent: WeakLink<T>,
        left: Link<T>,
        right: Link<T>,
    ) -> Link<T> {
        Some(Rc::new(RefCell::new(Node {
            key: key,
            color: color,
            parent: parent,
            left: left,
            right: right,
        })))
    }

    fn inorder_traverse(link: &Link<T>) -> Vec<T> {
        match link.as_ref() {
            Some(node) => [
                Node::inorder_traverse(&node.borrow().left),
                vec![node.borrow().key],
                Node::inorder_traverse(&node.borrow().right),
            ]
            .concat(),
            None => vec![],
        }
    }

    fn push(&mut self, key: T) {}
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;

    fn dummy_tree() -> RedBlackTree<i8> {
        //     1
        //    / \
        //   2   5
        //  / \
        // 3   4
        let node1 = Rc::new(RefCell::new(Node {
            key: 1,
            color: Color::Black,
            parent: None,
            left: None,
            right: None,
        }));
        let node2 = Rc::new(RefCell::new(Node {
            key: 2,
            color: Color::Black,
            parent: Some(Rc::downgrade(&node1)),
            left: None,
            right: None,
        }));
        let node3 = Rc::new(RefCell::new(Node {
            key: 3,
            color: Color::Black,
            parent: Some(Rc::downgrade(&node2)),
            left: None,
            right: None,
        }));
        let node4 = Rc::new(RefCell::new(Node {
            key: 4,
            color: Color::Black,
            parent: Some(Rc::downgrade(&node2)),
            left: None,
            right: None,
        }));
        let node5 = Rc::new(RefCell::new(Node {
            key: 5,
            color: Color::Black,
            parent: Some(Rc::downgrade(&node1)),
            left: None,
            right: None,
        }));
        node2.borrow_mut().left = Some(node3);
        node2.borrow_mut().right = Some(node4);
        node1.borrow_mut().left = Some(node2);
        node1.borrow_mut().right = Some(node5);
        RedBlackTree { root: Some(node1) }
    }

    #[test]
    fn inorder_traverse() {
        let tree = dummy_tree();
        let expected = tree.inorder_traverse();
        assert_eq!(vec![3, 2, 4, 1, 5], expected);
    }

    #[test]
    fn rotate() {
        let tree = dummy_tree();
        //     1
        //    / \
        //   2   5
        //  / \
        // 3   4
        tree.rotate(&tree.root, Direction::Left);
        assert_eq!(vec![3, 2, 4, 1, 5], tree.inorder_traverse());
    }

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
}
