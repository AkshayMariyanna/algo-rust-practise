//! # Algebra
//!
//! Common algorithms for efficient evaluation of common algebra problems

mod binary_exponentiation;
mod geometric_progression;
mod gcd;

pub use binary_exponentiation::*;
pub use geometric_progression::*;
pub use gcd::*;

/// LCM(a, b)
///
/// ```
/// let a = 10;
/// let b = 15;
///
/// assert_eq!(algebra::lcm(a, b), 30);
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}
