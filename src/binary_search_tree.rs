#[derive(Debug, PartialEq)]
enum BST<T> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Nil,
}

impl<T: Ord + PartialEq> BST<T> {
    fn new() -> Self {
        Self::Nil
    }

    fn is_nil(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }

    fn push(&mut self, new_value: T) {
        match self {
            Self::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => match *value > new_value {
                true => left.push(new_value),
                false => right.push(new_value),
            },
            Self::Nil => {
                *self = Self::Leaf {
                    value: new_value,
                    left: Box::new(Self::Nil),
                    right: Box::new(Self::Nil),
                }
            }
        }
    }

    // fn find(&self, target: T) -> &T {}
    // fn pop(&mut self) -> T {

    // }

    // fn delete(&mut self, value: T) -> bool { true }
}

#[cfg(test)]
mod test_binary_search_tree {
    use super::*;

    fn _eq_node_value<T: PartialEq>(node: &BST<T>, expected_value: T) -> bool {
        match node {
            BST::Leaf { value, .. } => *value == expected_value,
            BST::Nil => false,
        }
    }

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
        assert_eq!(tree, BST::Nil);

        // check if 5 is pushed
        tree.push(5);
        assert!(_eq_node_value(&tree, 5));

        // check if 2 is to the left of 5
        tree.push(2);
        assert!(_eq_node_value(&tree, 5));
        assert!(match tree {
            BST::Leaf {
                ref left,
                ref right,
                ..
            } => _eq_node_value(left, 2) && right.is_nil(),
            _ => false,
        });

        // check if 9 is to the right of 5
        tree.push(9);
        assert!(_eq_node_value(&tree, 5));
        assert!(match tree {
            BST::Leaf {
                ref left,
                ref right,
                ..
            } => _eq_node_value(left, 2) && _eq_node_value(right, 9),
            _ => false,
        });

        // check if 8 is to the left of 9
        tree.push(8);
        assert!(_eq_node_value(&tree, 5));
        assert!(match tree {
            BST::Leaf {
                ref left,
                ref right,
                ..
            } => _eq_node_value(left, 2) && _eq_node_value(right, 9),
            _ => false,
        });
        assert!(match tree {
            BST::Leaf { right, .. } => match *right {
                BST::Leaf { ref left, .. } => _eq_node_value(left, 8),
                _ => false,
            },
            _ => false,
        });
    }

    // #[test]
    // fn find() {
    // println!("");
    // }

    // #[test]
    // fn pop() {
    // let mut tree = _new_tree();

    // assert_eq!(tree.pop(), 2);
    //     assert_eq!(tree.value, 5);
    //     assert!(tree.left.is_none());
    //     assert!(_eq_node_value(&tree.right, 9));
    //     assert!(
    //         match tree.right {
    //             Some(right) => _eq_node_value(&right.left, 8),
    //             None => false,
    //         }
    //     );
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
    // }
}
