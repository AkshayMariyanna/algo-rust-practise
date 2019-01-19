/// GCD(a, b)
///
/// [CP - Algos](https://cp-algorithms.com/algebra/euclid-algorithm.html)
/// gcd(a, b) = {
///   if b = 0 => a
///   else     => gcd(b, a mod b)
/// }
///
/// Euclidean algorithm
/// ```
/// let a = 25;
/// let b = 10;
/// assert_eq!(algebra::gcd(a, b), 5);
///
/// let a = 15;
/// let b = 16;
/// assert_eq!(algebra::gcd(a, b), 1);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

/// GCD(a, b)
///
/// [CP - Algos](https://cp-algorithms.com/algebra/euclid-algorithm.html)
/// gcd(a, b) = {
///   if b = 0 => a
///   else     => gcd(b, a mod b)
/// }
///
/// Euclidean algorithm
/// ```
/// let a = 25;
/// let b = 10;
/// assert_eq!(algebra::gcd_non_recursive(a, b), 5);
///
/// let a = 15;
/// let b = 16;
/// assert_eq!(algebra::gcd_non_recursive(a, b), 1);
/// ```
pub fn gcd_non_recursive(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a = a % b;
        let temp = a;
        a = b;
        b = temp;
    }

    a
}

