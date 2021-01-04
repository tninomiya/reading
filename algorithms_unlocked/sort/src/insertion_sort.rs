use std::cmp::Ord;
use std::fmt::Debug;

/// insertion sort
/// - input
///   - a: target array
///   - n: number of elements in a
pub fn sort<T>(a: &mut [T], n: usize)
where
    T: Ord + Debug,
{
    for i in 1..n {
        let mut j = i;
        while j > 0 && a[j - 1] > a[j] {
            a.swap(j - 1, j);
            j -= 1;
        }
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
