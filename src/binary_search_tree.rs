type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, Copy)]
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

    fn push_or_insert(link: &mut Link<T>, new_value: T) {
        match link {
            Some(node) => node.push(new_value),
            None => *link = Self::new_link(new_value),
        }
    }

    fn push(&mut self, new_value: T) {
        if self.value > new_value {
            Self::push_or_insert(&mut self.left, new_value)
        } else {
            Self::push_or_insert(&mut self.right, new_value)
        }
    }

    fn find_or_none(link: &Link<T>, target: T) -> Option<&Node<T>> {
        match link {
            Some(node) => node.find(target),
            None => None,
        }
    }

    fn find(&self, target: T) -> Option<&Node<T>> {
        if self.value == target {
            Some(self)
        } else if self.value > target {
            Self::find_or_none(&self.left, target)
        } else {
            Self::find_or_none(&self.right, target)
        }
    }

    fn min_node_or_none(link: &Link<T>) -> Option<&Node<T>> {
        match link {
            Some(node) => node.min_node(),
            None => None,
        }
    }

    fn min_node(&self) -> Option<&Node<T>> {
        match self.left {
            Some(ref node) => node.min_node(),
            None => Some(self),
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

    fn find(&self, target: T) -> Option<&Node<T>> {
        Node::find_or_none(&self.root, target)
    }

    fn min_node(&self) -> Option<&Node<T>> {
        Node::min_node_or_none(&self.root)
    }

    fn pop_min(&mut self) -> Option<T> {
        Self::min_node(&mut self).map(|x| x.value)
    }
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
    }

    #[test]
    fn find() {
        let tree1: BST<i8> = _new_tree();
        let tree2: BST<i8> = _new_tree();

        // check if root is none
        //    N
        //   / \
        assert_eq!(BST::<i8>::new().root, None);

        // check if get ref. of root.
        //   [5]
        //   / \
        //  2   9
        //     /
        //    8
        assert_eq!(tree1.find(5), tree2.root.as_deref());

        // check if get ref. of the left node.
        //    5
        //   / \
        // [2]  9
        //     /
        //    8
        assert_eq!(
            tree1.find(2),
            tree2.root.as_deref().unwrap().left.as_deref()
        );

        // check if get ref. of the right node.
        //    5
        //   / \
        //  2  [9]
        //     /
        //    8
        assert_eq!(
            tree1.find(9),
            tree2.root.as_deref().unwrap().right.as_deref()
        );

        // check if get ref. of the deeper node.
        //    5
        //   / \
        //  2   9
        //     /
        //   [8]
        assert_eq!(
            tree1.find(8),
            tree2
                .root
                .as_deref()
                .unwrap()
                .right
                .as_deref()
                .unwrap()
                .left
                .as_deref()
        );
    }

    #[test]
    fn min_node() {
        let tree: BST<i8> = _new_tree();

        // check if root is none
        //    N
        //   / \
        assert_eq!(BST::<i8>::new().min_node(), None);

        // check when Node has a value and no left and no right.
        //   [2]
        //   / \
        //  N   N
        assert_eq!(
            tree.root
                .as_deref()
                .unwrap()
                .left
                .as_deref()
                .unwrap()
                .min_node(),
            Some(&Node::<i8>::new(2))
        );

        // check when Node has a value and no left and no right.
        //    5
        //   / \
        // [2]  9
        assert_eq!(tree.min_node(), Some(&Node::<i8>::new(2)));
    }

    fn pop() {
        let mut tree: BST<i8> = _new_tree();

        assert_eq!(tree.pop(), BST::new(2));
        assert_eq!(tree.pop(), BST::new_leaf(5));
        assert_eq!(tree.pop(), BST::new_leaf(8));
        assert_eq!(tree.pop(), BST::new_leaf(9));
        assert!(tree.pop().is_nil());
    }

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
