pub fn safe_subtract(n: usize, sub: usize) -> usize {
    if n < sub {
        0
    } else {
        n - sub
    }
}
