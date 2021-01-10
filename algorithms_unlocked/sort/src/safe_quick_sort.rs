use super::helper;
use rand::Rng;
use std::cmp::Ord;
use std::fmt::Debug;

/// in-place quick sort
/// the sort prevents attack for partitioning by pivot selection
/// with median of random 3 values
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
    a.swap(tail, pivot(a, head, tail, rand::thread_rng()));
    let pos_pivot = tail;

    for u in head..tail {
        if a[u] <= a[pos_pivot] {
            a.swap(larger_h, u);
            larger_h += 1;
        }
    }

    a.swap(larger_h, pos_pivot);
    larger_h
}

fn pivot<T, R>(a: &[T], head: usize, tail: usize, mut rng: R) -> usize
where
    T: Ord,
    R: Rng,
{
    let n: usize = rng.gen_range(head..=tail);
    let m: usize = rng.gen_range(head..=tail);
    let l: usize = rng.gen_range(head..=tail);
    helper::median(a, n, m, l)
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
    fn empty_sort_test() {
        let a: &mut [i64] = &mut [];
        sort(a);
        assert_eq!(a, &[]);
    }
}
