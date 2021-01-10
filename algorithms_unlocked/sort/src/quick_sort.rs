use super::helper;
use std::cmp::Ord;
use std::fmt::Debug;

/// in-place quick sort
/// the sort doesn't prevent any attack for partitioning
pub fn sort<T>(a: &mut [T])
where
    T: Ord + Debug,
{
    quick_sort(a, 0, helper::safe_subtract(a.len(), 1));
}

/// - input
///   - a: target array
///   - p, r: starting and ending indices of a subarray of a
fn quick_sort<T>(a: &mut [T], head: usize, tail: usize)
where
    T: Ord + Debug,
{
    if head >= tail {
        return;
    }
    let pivot = partition(a, head, tail);
    quick_sort(a, head, helper::safe_subtract(pivot, 1));
    quick_sort(a, pivot + 1, tail);
}

fn partition<T>(a: &mut [T], head: usize, tail: usize) -> usize
where
    T: Ord + Debug,
{
    let mut larger_h = head;
    for u in head..tail {
        if a[u] <= a[tail] {
            a.swap(larger_h, u);
            larger_h += 1;
        }
    }

    a.swap(larger_h, tail);
    larger_h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_sort_test() {
        let a = &mut [12, 9];
        sort(a);
        assert_eq!(a, &[9, 12]);
    }

    #[test]
    fn sort_test() {
        let a = &mut [12, 9, 3, 7, 14, 11, 6, 2, 10, 5];
        sort(a);
        assert_eq!(a, &[2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);
    }

    #[test]
    fn tempty_sort_test() {
        let a: &mut [i64] = &mut [];
        sort(a);
        assert_eq!(a, &[]);
    }
}
