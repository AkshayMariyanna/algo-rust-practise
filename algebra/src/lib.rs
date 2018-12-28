//! # Algebra
//! 
//! Common algorithms for efficient evaluation of common algebra problems

/// Calculates a^n in O(log n)
/// 
/// [Resource - CP Algos](https://cp-algorithms.com/algebra/binary-exp.html)
/// 
/// ```
/// let a = 3;
/// let n = 13;
/// 
/// assert_eq!(algebra::binary_exponentiation(a, n), 1594323);
/// ```
pub fn binary_exponentiation(mut a: u64, mut n: u64) -> u64 {
    let mut res = 1;
    while n != 0 && a != 0 {
        if n & 1 == 1 {
            res *= a;
        }
        a *= a;
        n >>= 1;
    }

    res
}

/// Calculates a^n % m
/// 
/// [Resource - CP Algos](https://cp-algorithms.com/algebra/binary-exp.html)
/// 
/// ```
/// let a = 11;
/// let n = 9;
/// let m = 997;
/// 
/// assert_eq!(algebra::binary_exponentiation_mod(a, n, m), 817);
/// ```
pub fn binary_exponentiation_mod(mut a: u64, mut n: u64, m: u64) -> u64 {
    a %= m;
    let mut res = 1;
    while n != 0 && a != 0 {
        if n & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }

    res
}
