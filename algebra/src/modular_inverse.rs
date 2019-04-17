//! [Modular Inverse](https://cp-algorithms.com/algebra/module-inverse.html)
use crate::gcd::gcd_extended1;

/// Modular Inverse Using Extended Euclidean Algorithm
///
/// Finds modular inverse of a w.r.t m
/// ```
/// assert_eq!(algebra::modular_inverse::modular_inverse_extended_gcd(2, 4), None);
/// assert_eq!(algebra::modular_inverse::modular_inverse_extended_gcd(2, 5), Some(3));
/// ```
pub fn modular_inverse_extended_gcd(a: i64, m: i64) -> Option<i64> {
    let ((x, _), g) = gcd_extended1(a, m);
    match g {
        1 => Some((x % m + m) % m),
        _ => None
    }
}
