use std::ops::{Deref, DerefMut};

// type T = u64;
struct Prime {
    curr: u64,
    nums: Vec<u64>,
}

impl Iterator for Prime {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.curr += 1;
            let prime_factors: Vec<&u64> =
                self.nums.iter().filter(|&x| self.curr % x == 0).collect();
            if prime_factors.is_empty() {
                break;
            }
        }
        self.nums.push(self.curr);
        Some(self.curr)
    }
}

fn prime_numbers() -> Prime {
    Prime {
        curr: 1,
        nums: Vec::<u64>::new(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_to_101nth() {
        let expected: [u64; 1] = [2];
        let result: Vec<u64> = prime_numbers().take(1).collect();
        assert_eq!(result, expected);

        let expected: [u64; 26] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];
        let result: Vec<u64> = prime_numbers().take(26).collect();
        assert_eq!(result, expected);
    }
}
