//! # Algebra
//!
//! Algorithms for efficient evaluation of common algebra problems

pub mod binary_exponentiation;
pub mod geometric_progression;
pub mod gcd;
pub mod linear_diophantine;
pub mod fibonacci;
pub mod prime;
pub mod primality;
pub mod factorization;
pub mod modular_inverse;
pub mod garners_algorithm;

pub mod lcm {
    pub use super::gcd::gcd;
    /// LCM(a, b)
    ///
    /// ```
    /// let a = 10;
    /// let b = 15;
    ///
    /// assert_eq!(algebra::lcm::lcm(a, b), 30);
    /// ```
    pub fn lcm(a: u64, b: u64) -> u64 {
        (a / gcd(a, b)) * b
    }
}
