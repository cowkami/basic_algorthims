type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug)]
struct BST<T> {
    root: Link<T>,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.left == other.left && self.right == other.right
    }
}

impl<T: PartialEq> Eq for Node<T> {}

impl<T: PartialEq> PartialEq for BST<T> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<T: PartialEq> Eq for BST<T> {}

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

    fn find_or_none(link: &mut Link<T>, target: T) -> Option<&mut Node<T>> {
        match link {
            Some(node) => node.find(target),
            None => None,
        }
    }

    fn find(&mut self, target: T) -> Option<&mut Node<T>> {
        if self.value == target {
            Some(self)
        } else if self.value > target {
            Self::find_or_none(&mut self.left, target)
        } else {
            Self::find_or_none(&mut self.right, target)
        }
    }

    fn min_node_or_none(link: &mut Link<T>) -> Option<&Node<T>> {
        match link {
            Some(node) => node.min_node(),
            None => None,
        }
    }

    fn min_node(&mut self) -> Option<&Node<T>> {
        match self.left {
            Some(ref mut node) => node.min_node(),
            None => Some(self),
        }
    }

    fn pop_min_or_none(link: &mut Link<T>) -> Option<&Node<T>> {}

    fn pop_min(&mut self) -> Option<T> {
        match self.left {
            Some(ref mut node) => node.min_node(),
            None => Some(self),
        }
    }
}

impl<T: PartialOrd + Copy> BST<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn push(&mut self, new_value: T) {
        Node::push_or_insert(&mut self.root, new_value)
    }

    fn find(&mut self, target: T) -> Option<&mut Node<T>> {
        Node::find_or_none(&mut self.root, target)
    }

    fn min_node(&mut self) -> Option<&Node<T>> {
        Node::min_node_or_none(&mut self.root)
    }

    fn pop_min(&mut self) -> Option<T> {
        Node::pop_min_or_none(&mut self.root)
    }
}

#[cfg(test)]
mod test {
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
        let mut tree1: BST<i8> = _new_tree();
        let mut tree2: BST<i8> = _new_tree();

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
        assert_eq!(tree1.find(5), tree2.root.as_deref_mut());

        // check if get ref. of the left node.
        //    5
        //   / \
        // [2]  9
        //     /
        //    8
        assert_eq!(
            tree1.find(2),
            tree2.root.as_deref_mut().unwrap().left.as_deref_mut()
        );

        // check if get ref. of the right node.
        //    5
        //   / \
        //  2  [9]
        //     /
        //    8
        assert_eq!(
            tree1.find(9),
            tree2.root.as_deref_mut().unwrap().right.as_deref_mut()
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
                .as_deref_mut()
                .unwrap()
                .right
                .as_deref_mut()
                .unwrap()
                .left
                .as_deref_mut()
        );
    }

    #[test]
    fn min_node() {
        let mut tree: BST<i8> = _new_tree();

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
                .as_deref_mut()
                .unwrap()
                .left
                .as_deref_mut()
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

    #[test]
    fn pop_min() {
        let mut tree: BST<i8> = _new_tree();

        assert_eq!(tree.pop_min(), Some(2));
        assert_eq!(tree.pop_min(), Some(5));
        // assert_eq!(tree.pop_min(), Some(8));
        // assert_eq!(tree.pop_min(), Some(9));
        // assert_eq!(tree.pop_min(), None);
    }
}
