use std::cmp::Ord;
use std::fmt::Debug;
use std::mem;

/// efficient linear search with sentinel
/// comparing better_linear_search, this search reduces the number of check per iteration
/// - input
///   - a: target array
///   - n: number of elements in a
///   - x: target element
pub fn search<T>(a: &mut [T], n: usize, x: T) -> Option<usize>
where
    T: Ord + Debug + Clone,
{
    if n == 0 {
        return None;
    }
    let last = mem::replace(&mut a[n - 1], x.clone());
    let mut i = 0;
    while !a[i].eq(&x) {
        i += 1;
    }
    a[n - 1] = last;
    if i < (n - 1) || a[n - 1].eq(&x) {
        return Some(i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::search;
    #[test]
    fn search_ordered_list_test() {
        let a = &mut [1, 2, 3, 4, 5, 6];
        assert_eq!(search(a, 6, 3), Some(2));
        assert_eq!(search(a, 6, 1), Some(0));
        assert_eq!(search(a, 6, 0), None);
    }
    #[test]
    fn search_unordered_list_test() {
        let a = &mut [4, 1, 2, 5, 6, 3];
        assert_eq!(search(a, 6, 3), Some(5));
        assert_eq!(search(a, 6, 1), Some(1));
        assert_eq!(search(a, 6, 0), None);
    }
    #[test]
    fn search_empty_list_test() {
        let a = &mut [];
        assert_eq!(search(a, 0, 3), None);
        assert_eq!(search(a, 0, 1), None);
        assert_eq!(search(a, 0, 0), None);
    }
}
