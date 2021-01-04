use std::cmp::Ord;
use std::fmt::Debug;

/// merge sort
/// - input
///   - a: target array
///   - p, r: starting and ending indices of a subarray of a
pub fn sort<T>(a: &mut [T], p: usize, r: usize)
where
    T: Ord + Debug + Clone,
{
    println!("divide: p: {}, r: {}", p, r);
    // subarray doesn't have multiple elements to be sorted
    if p >= r {
        return;
    }
    let q = (p + r) / 2;
    println!("pivot: {}", q);
    sort(a, p, q);
    sort(a, q + 1, r);
    merge(a, p, q, r);
}

fn merge<T>(a: &mut [T], low: usize, pivot: usize, high: usize)
where
    T: Ord + Debug + Clone,
{
    let former = &a[low..=pivot].to_vec();
    let later = &a[(pivot + 1)..=high].to_vec();
    let mut former_index = 0;
    let mut later_index = 0;
    let former_len = pivot - low;
    let later_len = high - pivot;
    for k in low..=high {
        // pick up the next element from the former part of subarray
        if (former_index <= former_len)
            && (
                // the later part is already consumed.
                later_index >= later_len ||
                // the next element in the former part is smaller than the one in the later part
                former[former_index] <= later[later_index]
            )
        {
            a[k] = former[former_index].clone();
            former_index += 1;
        } else {
            // pick up an element from the later part of subarray
            a[k] = later[later_index].clone();
            later_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touple_sort_test() {
        let a = &mut [12, 9];
        let n = a.len();
        sort(a, 0, n - 1);
        assert_eq!(a, &[9, 12]);
    }

    #[test]
    fn sort_test() {
        let a = &mut [12, 9, 3, 7, 14, 11, 6, 2, 10, 5];
        let n = a.len();
        sort(a, 0, n - 1);
        assert_eq!(a, &[2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);
    }
}
