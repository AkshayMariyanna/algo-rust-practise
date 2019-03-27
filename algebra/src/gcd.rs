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

/// GCD(a, b) Extended
///
/// Calculates coefficients (x, y) for given (a, b) such that a*x + b*y = GCD(a, b)
///
/// [CP - Algos](https://cp-algorithms.com/algebra/extended-euclid-algorithm.html)
///
/// Extended Euclidean algorithm
/// ```
/// let b = 25;
/// let a = 10;
/// let (x, y) = algebra::gcd_extended(a, b);
/// assert_eq!(x * a + y * b, 5);
///
/// let a = 15;
/// let b = 16;
/// let (x, y) = algebra::gcd_extended(a, b);
/// assert_eq!(x * a + y * b, 1);
/// ```
pub fn gcd_extended(a: i64, b: i64) -> (i64, i64) {
    if a == 0 {
        (0, 1)
    } else {
        let (x1, y1) = gcd_extended(b % a, a);
        (y1 - (b / a) * x1, x1)
    }
}

/// GCD(a, b) Extended
///
/// Calculates coefficients (x, y) and gcd g for given (a, b) such that a*x + b*y = g
/// ```
/// let b = 25;
/// let a = 10;
/// let ((x, y), g) = algebra::gcd_extended1(a, b);
/// assert_eq!(x * a + y * b, g);
///
/// let a = 15;
/// let b = 16;
/// let ((x, y), g) = algebra::gcd_extended1(a, b);
/// assert_eq!(x * a + y * b, g);
/// ```
pub fn gcd_extended1(a: i64, b: i64) -> ((i64, i64), i64) {
    if a == 0 {
        ((0, 1), b)
    } else {
        let ((x1, y1), g) = gcd_extended1(b % a, a);
        ((y1 - (b / a) * x1, x1), g)
    }
}
