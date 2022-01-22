#[derive(Debug)]
struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T: PartialOrd> BinaryTree<T> {
    fn new(new_value: T) -> Self {
        Self {
            value: new_value,
            left: None,
            right: None,
        }
    }
    fn push(&mut self, new_value: T) {
        if self.value > new_value {
            match self.left {
                Some(ref mut left) => left.push(new_value),
                None => self.left = Some(Box::new(BinaryTree::<T>::new(new_value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.push(new_value),
                None => self.right = Some(Box::new(BinaryTree::<T>::new(new_value))),
            }
        } 
    }
    fn pop(&mut self) {}
    fn delete(&mut self, value: T) {}
}
 

#[cfg(test)]
mod test_binary_search_tree {
    use super::*;

    fn _eq_node_value<T: PartialEq>(node: &Option<Box<BinaryTree<T>>>, value: T) -> bool {
        match node {
            Some(ref n) => n.value == value,
            None => false,
        }
    }

    #[test]
    fn push() {
        let mut tree = BinaryTree::<i8>::new(5);

        // check initialization
        assert_eq!(tree.value, 5);
        assert!(tree.left.is_none());
        assert!(tree.right.is_none());

        // check if 2 is to the left of 5
        tree.push(2);
        assert_eq!(tree.value, 5);
        assert!(_eq_node_value(&tree.left, 2));
        assert!(tree.right.is_none());

        // check if 9 is to the right of 5
        tree.push(9);
        assert_eq!(tree.value, 5);
        assert!(_eq_node_value(&tree.left, 2));
        assert!(_eq_node_value(&tree.right, 9));

        // check if 8 is to the left of 9
        tree.push(8);
        assert_eq!(tree.value, 5);
        assert!(_eq_node_value(&tree.left, 2));
        assert!(_eq_node_value(&tree.right, 9));
        assert!(
            match tree.right {
                Some(right) => _eq_node_value(&right.left, 8),
                None => false,
            }
        );
    } 

    #[test]
    fn pop() {} 

    #[test]
    fn delete() {}
}