#[allow(dead_code)]
fn factorial(n: i64) -> i64 {
    if n < 0 {
        panic!("given number must be positive or equal to 0")
    }
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

use std::cmp::Ord;
use std::fmt::Debug;

/// linear search with recursive implementation
/// - input
///   - a: target array
///   - n: number of elements in a
///   - i: begin index of subarray
///   - x: target element
pub fn search<T>(a: &[T], n: usize, i: usize, x: T) -> Option<usize>
where
    T: Ord + Debug,
{
    if i >= n {
        return None;
    } else if a[i] == x {
        return Some(i);
    } else {
        return search(a, n, i + 1, x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_test() {
        assert_eq!(factorial(5), 120);
    }

    #[cfg(test)]
    mod tests {
        use super::search;
        #[test]
        fn search_ordered_list_test() {
            let a = &[1, 2, 3, 4, 5, 6];
            assert_eq!(search(a, 6, 0, 3), Some(2));
            assert_eq!(search(a, 6, 0, 1), Some(0));
            assert_eq!(search(a, 6, 0, 0), None);
        }
        #[test]
        fn search_unordered_list_test() {
            let a = &[4, 1, 2, 5, 6, 3];
            assert_eq!(search(a, 6, 0, 3), Some(5));
            assert_eq!(search(a, 6, 0, 1), Some(1));
            assert_eq!(search(a, 6, 0, 0), None);
        }
        #[test]
        fn search_empty_list_test() {
            let a = &[];
            assert_eq!(search(a, 0, 0, 3), None);
            assert_eq!(search(a, 0, 0, 1), None);
            assert_eq!(search(a, 0, 0, 0), None);
        }
    }
}
