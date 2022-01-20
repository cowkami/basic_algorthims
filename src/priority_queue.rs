struct Heap<T> {
    data: Vec<T>,
    len: usize
}

impl<T: PartialOrd + Copy> Heap<T> {
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
    fn pop(&mut self) -> T {
        let ret = self.data[0];
        self.data[0] = self.data[self.len - 1];
        self.len -= 1;

        let mut i = 0;
        while i < self.len / 2 {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            if self.data[i] > self.data[l] {
                self.data.swap(i, l);
                i = l;
            } else if self.data[i] > self.data[r] {
                self.data.swap(i, r);
                i = r;
            } else { break }
        }
        ret
    }
}


#[cfg(test)]
mod test_priority_queue {
    use super::*;

    #[test]
    fn construct() {
        type T = i32;
        let heap = Heap::<T>::new();

        assert_eq!(heap.data, Vec::<T>::new());
        assert_eq!(heap.len, 0);
    }

    #[test]
    fn push() {
        let mut heap = Heap::<f32>::new();

        // check just push
        heap.push(1.0);
        assert_eq!(heap.data[0], 1.0);
        assert_eq!(heap.len, 1);

        // check if 2.0 is after 1.0
        heap.push(2.1);
        assert_eq!(heap.data[1], 2.1);
        assert_eq!(heap.len, 2);

        // check if 0.9 is before 1.0
        heap.push(0.9);
        assert_eq!(heap.data[0], 0.9);
        assert_eq!(heap.data[1], 2.1);
        assert_eq!(heap.data[2], 1.0);
        assert_eq!(heap.len, 3);

        // check if 0.8 is before 0.9
        heap.push(0.8);
        assert_eq!(heap.data[0], 0.8);
        assert_eq!(heap.data[1], 0.9);
        assert_eq!(heap.data[2], 1.0);
        assert_eq!(heap.data[3], 2.1);
        assert_eq!(heap.len, 4);
    }

    #[test]
    fn pop() {
        let mut heap = Heap::<f32>::new();

        heap.push(1.0);
        heap.push(2.1);
        heap.push(0.9);
        heap.push(0.8);
        assert_eq!(heap.len, 4);

        assert_eq!(heap.pop(), 0.8);
        assert_eq!(heap.len, 3);

        assert_eq!(heap.pop(), 0.9);
        assert_eq!(heap.len, 2);

        assert_eq!(heap.pop(), 1.0);
        assert_eq!(heap.len, 1);

        assert_eq!(heap.pop(), 2.1);
        assert_eq!(heap.len, 0);
    }
}