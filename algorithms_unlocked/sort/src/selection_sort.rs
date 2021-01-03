use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

/// selection sort
/// - input
///   - a: target array
///   - n: number of elements in a
pub fn sort<T>(a: &mut [T], n: usize)
where
    T: Ord + Debug,
{
    for i in 0..n {
        let mut smallest = i;
        for j in i..n {
            if let Ordering::Less = a[j].cmp(&a[smallest]) {
                smallest = j;
            }
        }
        a.swap(i, smallest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let a = &mut [12, 9, 3, 7, 14, 11];
        let n = a.len();
        sort(a, n);
        assert_eq!(a, &[3, 7, 9, 11, 12, 14]);
    }
}
