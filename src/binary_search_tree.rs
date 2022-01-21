// use std::cmp;

struct BinaryTree<T> {
    value: Option<T>,
    left: Option<BinaryTree<T>>,
    right: Option<BinaryTree<T>>,
}

impl<T: PartialOrd> BinaryTree<T> {
    fn new() -> Self {
        Self { value: None, left: None, right: None }
    }
    fn push(&mut self, new_value: T) {
        if let Some(v) = self.value {
            if v < new_value {
                if let None = self.left {
                    self.left = Some(BinaryTree::<T>::new());
                }
                self.left.push(new_value);
            } else {
                if let None = self.right {
                    self.right = Some(BinaryTree::<T>::new());
                }
                self.right.push(new_value);
            }
        } else {
            self.value = Some(new_value);
        }
    }
    fn pop(&mut self) -> Option<T> { return self.value }
    fn delete(&mut self, value: T) {}
}
 

#[cfg(test)]
mod test_binary_search_tree {
    use super::*;

    #[test]
    fn push() {
        let mut tree = BinaryTree::<i8>::new();

        // check just push
        tree.push(5);
        assert_eq!(tree.value, Some(5));
        // assert!(let None = tree.left);
        // assert!(let None = tree.right);

        // check if 2 is to the left of 5
        // tree.push(2);
        // assert_eq!(tree.value, Some(5));
        // assert_eq!(tree.left.value, Some(2));
        // assert_eq!(tree.right, None);

        // check if 9 is to the right of 5
        // tree.push(9);
        // aassert_eq!(tree.value, Some(5));
        // assert_eq!(tree.left.value, Some(2));
        // assert_eq!(tree.right.value, Some(9));

        // check if 8 is to the left of 9
        // assert_eq!(tree.left.left, 8);
    } 

    #[test]
    fn pop() {} 

    #[test]
    fn delete() {}
}