use std::cmp;

struct Heap<T> {
    data: Vec<T>,
    len: usize
}

impl<T: cmp::PartialOrd> Heap<T> {
    fn new() -> Self {
        Self { data: Vec::<T>::new(), len: 0 }
    }
    fn push(&mut self, value: T) {
        self.data.push(value);
        self.len += 1;

        let mut i = self.len - 1;
        while i > 0 {
            let j = (i - 1) / 2;
            if self.data[i] < self.data[j] {
                self.data.swap(i, j);
            }
            i = j;
        }
    }
}
 

#[cfg(test)]
mod test_binary_search_tree {
    use super::*;

    #[test]
    fn push() {} 

    #[test]
    fn search() {} 

    #[test]
    fn delete() {}
}