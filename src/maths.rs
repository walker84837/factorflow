/// Returns the greatest common divisor of `a` and `b`.
/// If `b` is 0, `a` is returned.
/// # Example
/// ```
/// assert_eq!(gcd(10, 15), 5);
/// ```
pub fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 && b == 0 {
        panic!("GCD is undefined for both numbers being zero");
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

/// Returns the least common multiple of `a` and `b`.
/// # Example
/// ```
/// assert_eq!(lcm(10, 15), 30);
/// ```
pub fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 && b == 0 {
        panic!("LCM is undefined for both numbers being zero");
    }
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 15), 5);
        assert_eq!(gcd(10, 0), 10);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(1, 1), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(10, 15), 30);
        assert_eq!(lcm(10, 0), 0);
        assert_eq!(lcm(0, 10), 0);
        assert_eq!(lcm(1, 1), 1);
    }

    #[test]
    #[should_panic]
    fn test_gcd_division_by_zero() {
        gcd(0, 0);
    }

    #[test]
    #[should_panic]
    fn test_lcm_division_by_zero() {
        lcm(0, 0);
    }
}
