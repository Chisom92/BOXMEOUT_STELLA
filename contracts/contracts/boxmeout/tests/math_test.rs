#![cfg(test)]

use boxmeout::math::sqrt;

#[test]
fn test_zero_and_one() {
    assert_eq!(sqrt(0), 0);
    assert_eq!(sqrt(1), 1);
}

#[test]
fn test_perfect_squares() {
    for &s in &[2u128, 3, 4, 5, 10, 100, 1_000, 1_000_000, u32::MAX as u128] {
        let n = s * s;
        assert_eq!(sqrt(n), s, "sqrt({n}) should be {s}");
    }
}

#[test]
fn test_near_squares() {
    // Values just above a perfect square should still floor to s
    for &s in &[2u128, 3, 7, 15, 99, 999] {
        let n = s * s;
        assert_eq!(sqrt(n + 1), s, "sqrt({}+1) should be {s}", n);
        // (s+1)^2 - 1 = s^2 + 2s  →  still floors to s
        assert_eq!(sqrt(n + 2 * s), s, "sqrt({}) should be {s}", n + 2 * s);
    }
}

#[test]
fn test_floor_invariant() {
    // sqrt(n)^2 <= n < (sqrt(n)+1)^2 for a broad range
    for &n in &[
        2u128, 3, 5, 8, 24, 26, 99, 101, 9_999, 10_001,
        u32::MAX as u128, u64::MAX as u128,
    ] {
        let s = sqrt(n);
        assert!(s * s <= n, "s*s <= n failed for n={n}");
        assert!(n < (s + 1) * (s + 1), "n < (s+1)^2 failed for n={n}");
    }
}
