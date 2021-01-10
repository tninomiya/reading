pub fn safe_subtract(n: usize, sub: usize) -> usize {
    if n < sub {
        0
    } else {
        n - sub
    }
}

use std::cmp::Ord;

pub fn median<T>(a: &[T], x: usize, y: usize, z: usize) -> usize
where
    T: Ord,
{
    let e_x = &a[x];
    let e_y = &a[y];
    let e_z = &a[z];

    if e_x >= e_y {
        if e_x <= e_z {
            x
        } else if e_y > e_z {
            y
        } else {
            z
        }
    } else {
        if e_x >= e_z {
            x
        } else if e_y <= e_z {
            y
        } else {
            z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn median_unique() {
        let a = &[1, 2, 3, 4, 5];
        assert_eq!(a[median(a, 0, 1, 2)], 2);
        assert_eq!(a[median(a, 3, 0, 4)], 4);
    }

    #[test]
    fn median_same_pos() {
        let a = &[1, 2, 3];
        assert_eq!(a[median(a, 0, 0, 0)], 1);
        assert_eq!(a[median(a, 1, 1, 2)], 2);
    }

    #[test]
    fn median_same_elements() {
        let a = &[1, 1, 1, 2, 2, 3];
        assert_eq!(a[median(a, 0, 1, 3)], 1);
        assert_eq!(a[median(a, 3, 4, 0)], 2);
        assert_eq!(a[median(a, 1, 1, 3)], 1);
    }
}
