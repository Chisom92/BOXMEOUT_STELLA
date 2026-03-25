/// Integer floor square root via Newton-Raphson.
///
/// Returns the largest integer `s` such that `s * s <= n`.
/// Terminates in O(log n) iterations.
pub fn sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    // Initial estimate: highest bit position gives a good starting point.
    let mut x = 1u128 << ((128 - n.leading_zeros()) / 2);

    loop {
        let x1 = (x + n / x) / 2;
        if x1 >= x {
            return x;
        }
        x = x1;
    }
}

#[cfg(test)]
mod tests {
    use super::sqrt;

    #[test]
    fn test_zero_and_one() {
        assert_eq!(sqrt(0), 0);
        assert_eq!(sqrt(1), 1);
    }

    #[test]
    fn test_perfect_squares() {
        for s in [2u128, 3, 4, 5, 10, 100, 1_000, 1_000_000, u32::MAX as u128] {
            let n = s * s;
            assert_eq!(sqrt(n), s, "sqrt({n}) should be {s}");
        }
    }

    #[test]
    fn test_near_squares() {
        // Values just above a perfect square
        for s in [2u128, 3, 7, 15, 99, 999] {
            let n = s * s;
            // n+1 and n+2*s (= (s+1)^2 - 1) should still floor to s
            assert_eq!(sqrt(n + 1), s);
            assert_eq!(sqrt(n + 2 * s), s); // (s+1)^2 - 1
        }
    }

    #[test]
    fn test_floor_invariant() {
        // sqrt(n)^2 <= n < (sqrt(n)+1)^2 for a range of values
        for n in [2u128, 3, 5, 8, 24, 26, 99, 101, 9999, 10001, u64::MAX as u128] {
            let s = sqrt(n);
            assert!(s * s <= n, "s*s <= n failed for n={n}");
            assert!(n < (s + 1) * (s + 1), "n < (s+1)^2 failed for n={n}");
        }
    }
}
