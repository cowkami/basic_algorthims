type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

struct BST<T> {
    root: Link<T>,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.left == other.left && self.right == other.right
    }
}
impl<T: PartialEq> Eq for Node<T> {}

impl<T: PartialOrd> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value: value,
            left: None,
            right: None,
        }
    }

    fn new_link(value: T) -> Link<T> {
        Some(Box::new(Node::new(value)))
    }

    fn push_or_insert(opt_node: &mut Link<T>, new_value: T) {
        match opt_node {
            Some(node) => {
                node.push(new_value);
            }
            None => *opt_node = Self::new_link(new_value),
        }
    }

    fn push(&mut self, new_value: T) {
        match self.value > new_value {
            true => Self::push_or_insert(&mut self.left, new_value),
            false => Self::push_or_insert(&mut self.right, new_value),
        }
    }
}

impl<T: PartialOrd> BST<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn push(&mut self, new_value: T) {
        Node::push_or_insert(&mut self.root, new_value)
    }

    // fn find(&mut self, target: T) -> &Self {
    //     match self {
    //         Self::Node { value, .. } if *value == target => self,
    //         Self::Node { value, left, right } => match *value > target {
    //             true => left.find(target),
    //             false => right.find(target),
    //         },
    //         Self::Nil => self,
    //     }
    // }

    // fn min(self) -> Self {
    //     match self {
    //         Self::Node { .. } => self.0.min(),
    //         Self::Nil => Self::Nil,
    //     }
    // }
}

#[cfg(test)]
mod test_binary_search_tree {
    use super::*;

    fn _new_tree() -> BST<i8> {
        let mut tree = BST::<i8>::new();
        tree.push(5);
        tree.push(2);
        tree.push(9);
        tree.push(8);
        tree
    }

    #[test]
    fn push() {
        let mut tree = BST::<i8>::new();

        // check initialization
        assert_eq!(tree.root, None);

        // check if 5 is pushed
        tree.push(5);

        //   5
        //  / \
        // N   N
        assert_eq!(tree.root.as_ref().unwrap().value, 5);
        assert_eq!(tree.root.as_ref().unwrap().left, None);
        assert_eq!(tree.root.as_ref().unwrap().right, None);

        // check if 2 is to the left of 5
        tree.push(2);

        //     5
        //    / \
        //   2   N
        //  / \
        // N   N
        assert_eq!(tree.root.as_ref().unwrap().value, 5);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().value, 2);
        assert_eq!(
            tree.root.as_ref().unwrap().left.as_ref().unwrap().left,
            None
        );
        assert_eq!(
            tree.root.as_ref().unwrap().left.as_ref().unwrap().right,
            None
        );
        assert_eq!(tree.root.as_ref().unwrap().right, None);

        // check if 9 is to the right of 5
        tree.push(9);

        //   5
        //  / \
        // 2   9
        assert_eq!(tree.root.as_ref().unwrap().value, 5);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().value, 2);
        assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().value, 9);

        // check if 8 is to the left of 9
        tree.push(8);

        //   5
        //  / \
        // 2   9
        //    /
        //   8
        assert_eq!(tree.root.as_ref().unwrap().value, 5);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().value, 2);
        assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().value, 9);
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            8
        );

        // #[test]
        // fn find() {
        //     let mut tree: BST<i8> = _new_tree();

        //     assert!(_eq_node_value(&tree.find(5), 5));
        //     assert!(_eq_node_value(&tree.find(2), 2));
        //     assert!(_eq_node_value(&tree.find(9), 9));
        //     assert!(_eq_node_value(&tree.find(8), 8));
        //     assert!(!_eq_node_value(&tree.find(4), 4));
        // }

        // #[test]
        // fn pop() {
        //     let mut tree: BST<i8> = _new_tree();

        //     assert_eq!(tree.pop(), BST::new_leaf(2));
        //     assert_eq!(tree.pop(), BST::new_leaf(5));
        //     assert_eq!(tree.pop(), BST::new_leaf(8));
        //     assert_eq!(tree.pop(), BST::new_leaf(9));
        //     assert!(tree.pop().is_nil());
        // }

        // #[test]
        // fn delete() {
        // let mut tree1 = new_data();

        // // check if returns false when no 6 value on any node
        // assert!(!tree1.delete(6));

        // // check if replaces nodes keeping smaller left
        // assert!(tree1.delete(5));
        // assert!(_eq_node_value(&tree.left, 2));
        // assert!(tree.right.is_none());
    }
}
