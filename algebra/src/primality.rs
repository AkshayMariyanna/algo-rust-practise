//! [Primality Tests](https://cp-algorithms.com/algebra/primality_tests.html)

use rand::{thread_rng, RngCore};

use crate::binary_exponentiation::binary_exponentiation_mod;

/// Trial Division
/// ```
/// assert_eq!(algebra::primality::is_prime(13), true);
/// assert_eq!(algebra::primality::is_prime(77), false);
/// ```
pub fn is_prime(x: i64) -> bool {
    for d in 2..x {
        if d * d > x {
            break;
        }
        if x % d == 0 {
            return false
        }
    }
    return true;
}

/// Fermat Primality Test
/// ```
/// assert_eq!(algebra::primality::probably_prime_fermat(103, 5), true);
/// assert_eq!(algebra::primality::probably_prime_fermat(561, 558), false); // Carmichael number
/// ```
pub fn probably_prime_fermat(n: u64, iter: u64) -> bool {
    if n < 4 {
        n == 2 || n == 3
    } else {
        let mut rng = thread_rng();
        for _ in 0..iter {
            let a = 2 + rng.next_u64() % (n - 3);
            if binary_exponentiation_mod(a, n - 1, n) != 1 {
                return false;
            }
        }
        true
    }
}

fn check_composite(n: u64, a: u64, d: u64, s: i32) -> bool {
    let mut x = binary_exponentiation_mod(a, d, n);
    if x == 1 || x == n - 1 {
        return false;
    }
    if s > 1 {
        for _ in 1..s {
            x = (((x * x) as u128) % n as u128) as u64;
            if x == n -1 {
                return false;
            }
        }
    }
    true
}

/// Miller Rabin Primality Test
/// ```
/// assert_eq!(algebra::primality::non_deterministic_millerrabin(103), true);
/// assert_eq!(algebra::primality::non_deterministic_millerrabin(561), false); // Miller Rabin doesnt have numbers like Carmichael numbers
/// ```
pub fn non_deterministic_millerrabin(n: u64) -> bool {
    const ITER: u32 = 5;
    if n < 4 {
        return n == 2 || n == 3
    }

    let mut s = 0;
    let mut d = n - 1;
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }

    let mut rng = thread_rng();
    for _ in 0..ITER {
        let a = 2 + rng.next_u64() % (n - 3);
        if check_composite(n, a, d, s) {
            return false;
        }
    }
    true
}

/// Miller Rabin Primality Test deterministic for 64bit integers
/// ```
/// assert_eq!(algebra::primality::non_deterministic_millerrabin(103), true);
/// assert_eq!(algebra::primality::non_deterministic_millerrabin(561), false); // Miller Rabin doesnt have numbers like Carmichael numbers
/// ```
pub fn deterministic_millerrabin(n: u64) -> bool {
    if n < 4 {
        return n == 2 || n == 3
    }

    let mut s = 0;
    let mut d = n - 1;
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }

    for &a in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].iter() {
        if n == a {
            return true;
        }
        if check_composite(n, a, d, s) {
            return false;
        }
    }
    true
}
