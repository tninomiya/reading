use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

/// binary search
/// its signature follows the one listed in the book
/// - input
///   - a: target array, its elements must be sorted with ascendant order
///   - n: number of elements in a
///   - x: target element
pub fn search<T>(a: &[T], n: usize, x: T) -> Option<usize>
where
    T: Ord + Debug,
{
    let mut low = 0;
    let mut high = n as i32 - 1;
    while low <= high {
        let mid = (low + high) / 2;
        let pivot = mid as usize;

        match a[pivot].cmp(&x) {
            Ordering::Equal => return Some(pivot),
            Ordering::Greater => high = mid - 1,
            Ordering::Less => low = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::search;
    #[test]
    fn search_ordered_list_test() {
        let a = &[1, 2, 3, 4, 5, 6];

        assert_eq!(search(a, 6, 3), Some(2));
        assert_eq!(search(a, 6, 1), Some(0));
        assert_eq!(search(a, 6, 0), None);
    }

    #[test]
    fn search_empty_list_test() {
        let a = &[];
        assert_eq!(search(a, 0, 3), None);
        assert_eq!(search(a, 0, 1), None);
        assert_eq!(search(a, 0, 0), None);
    }
}
