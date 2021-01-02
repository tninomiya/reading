use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

/// simple linear search
/// - input
///   - a: target array
///   - n: number of elements in a
///   - x: target element
pub fn search<T>(a: &[T], n: usize, x: T) -> Option<usize>
where
    T: Ord + Debug,
{
    let mut answer = None;
    for i in 0..n {
        match a[i].partial_cmp(&x) {
            Some(Ordering::Equal) => answer = Some(i),
            _ => continue,
        }
    }
    answer
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
    fn search_unordered_list_test() {
        let a = &[4, 1, 2, 5, 6, 3];
        assert_eq!(search(a, 6, 3), Some(5));
        assert_eq!(search(a, 6, 1), Some(1));
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
