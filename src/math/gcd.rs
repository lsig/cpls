use std::cmp;

pub fn gcd(a: usize, b: usize) -> usize {
    let mut x = cmp::max(a, b);
    let mut y = cmp::min(a, b);

    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
}
