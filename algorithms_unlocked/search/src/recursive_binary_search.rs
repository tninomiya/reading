use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

/// binary search with recursive implementation
/// its signature follows the one listed in the book
/// - input
///   - a: target array, its elements must be sorted with ascendant order
///   - n: number of elements in a
///   - x: target element
pub fn search<T>(a: &[T], low: i32, high: i32, x: T) -> Option<usize>
where
    T: Ord + Debug,
{
    if low > high || (a.len() == 0) {
        return None;
    }
    let mid = (low + high) / 2;
    let pivot = mid as usize;

    match a[pivot].cmp(&x) {
        Ordering::Equal => Some(pivot),
        Ordering::Greater => search(a, low, mid - 1, x),
        Ordering::Less => search(a, mid + 1, high, x),
    }
}

#[cfg(test)]
mod tests {
    use super::search;
    #[test]
    fn search_ordered_list_test() {
        let a = &[1, 2, 3, 4, 5, 6];

        assert_eq!(search(a, 0, 6, 3), Some(2));
        assert_eq!(search(a, 0, 6, 1), Some(0));
        assert_eq!(search(a, 0, 6, 0), None);
    }

    #[test]
    fn search_empty_list_test() {
        let a = &[];
        assert_eq!(search(a, 0, 0, 3), None);
        assert_eq!(search(a, 0, 0, 1), None);
        assert_eq!(search(a, 0, 0, 0), None);
    }
}
