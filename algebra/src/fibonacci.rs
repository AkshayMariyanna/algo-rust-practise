//! [Fibonacci](https://cp-algorithms.com/algebra/fibonacci-numbers.html)

/// Fast Doubling method
/// Caclulates fib(n) in O(log n)
/// ```
/// let n = 4;
/// let (n4, n5) = algebra::fibonacci::fibonacci(n);
/// assert_eq!(n4, 3);
/// assert_eq!(n5, 5);
/// ```
pub fn fibonacci(n: i64) -> (i64, i64) {
    if n == 0 {
        (0, 1)
    } else {
        let (k, k1) = fibonacci(n >> 1);
        let c = k * (2 * k1 - k);
        let d = k1*k1 + k * k;

        if n & 1 == 1 {
            (d, c + d)
        } else {
            (c, d)
        }
    }
}
