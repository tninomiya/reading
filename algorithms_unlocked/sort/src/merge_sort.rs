use super::helper;
use std::cmp::Ord;
use std::fmt::Debug;

/// merge sort
pub fn sort<T>(a: &mut [T])
where
    T: Ord + Debug + Clone,
{
    merge_sort(a, 0, helper::safe_subtract(a.len(), 1));
}

/// - input
///   - a: target array
///   - p, r: starting and ending indices of a subarray of a
fn merge_sort<T>(a: &mut [T], head: usize, tail: usize)
where
    T: Ord + Debug + Clone,
{
    if tail == head {
        return;
    }
    let pivot = (head + tail) / 2;
    merge_sort(a, head, pivot);
    merge_sort(a, pivot + 1, tail);
    merge(a, head, pivot, tail);
}

fn merge<T>(a: &mut [T], head: usize, pivot: usize, tail: usize)
where
    T: Ord + Debug + Clone,
{
    let left = a[head..=pivot].to_vec();
    let right = a[(pivot + 1)..=tail].to_vec();

    let mut left_h = 0;
    let mut right_h = 0;
    let left_t = left.len() - 1;
    let right_t = right.len() - 1;
    let mut i = 0;

    while right_h <= right_t || left_h <= left_t {
        if right_h > right_t || (left_h <= left_t && left[left_h] < right[right_h]) {
            a[head + i] = left[left_h].clone();
            left_h += 1;
        } else {
            a[head + i] = right[right_h].clone();
            right_h += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touple_sort_test() {
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
