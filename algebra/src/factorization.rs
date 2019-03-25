//! https://cp-algorithms.com/algebra/factorization.html

/// Trial Division
/// ```
/// assert_eq!(algebra::trial_division(60), vec![2, 2, 3, 5]);
/// assert_eq!(algebra::trial_division(210), vec![2, 3, 5, 7]);
/// ```
pub fn trial_division(mut n: u64) -> Vec<u64> {
    let mut factorization: Vec<u64> = Vec::new();

    for d in 2..n {
        if d * d > n {
            break;
        }

        while n % d == 0 {
            factorization.push(d);
            n /= d;
        }
    }

    if n > 1 {
        factorization.push(n);
    }

    factorization
}

/// Trial Division wheel optimized
/// ```
/// assert_eq!(algebra::trial_division_wheel(60), vec![2, 2, 3, 5]);
/// assert_eq!(algebra::trial_division_wheel(210), vec![2, 3, 5, 7]);
/// ```
pub fn trial_division_wheel(mut n: u64) -> Vec<u64> {
    let mut factorization: Vec<u64> = Vec::new();

    for &d in [2, 3, 5].iter() {
        while n % d == 0 {
            factorization.push(d);
            n /= d;
        }
    }

    let mut i = 0;
    let mut d = 7;
    static INCREMENTS: [u64; 8] = [4, 2, 4, 2, 4, 6, 2, 6];
    while d * d <= n {
        d += INCREMENTS[i];
        i += 1;
        while n % d == 0 {
            factorization.push(d);
            n /= d;
        }
        if i == 8 {
            i = 0;
        }
    }

    if n > 1 {
        factorization.push(n);
    }

    factorization
}

/// Trial Division precomputed primes
/// ```
/// assert_eq!(algebra::trial_division_primes_precomputed(35, &[2, 3, 5]), vec![5, 7]);
/// assert_eq!(algebra::trial_division_primes_precomputed(208, &[2, 3, 5, 7, 11, 13]), vec![2, 2, 2, 2, 13]);
/// ```
pub fn trial_division_primes_precomputed(mut n: u64, primes: &[u64]) -> Vec<u64> {
    let mut factorization = Vec::new();

    for &d in primes {
        if d * d > n {
            break;
        }

        while n % d == 0 {
            n /= d;
            factorization.push(d);
        }
    }
    if n > 1 {
        factorization.push(n);
    }

    factorization
}

/// Fermat's factorization method
/// ```
/// assert_eq!(algebra::fermat_factorization(49), 7);
/// assert_eq!(algebra::fermat_factorization(56), 4);
/// ```
pub fn fermat_factorization(n: u32) -> u32 {
    let mut a: u32 = (n as f64).sqrt().ceil() as u32;
    let mut b2: u32 = a * a - n;
    let mut b: u32 = (b2 as f64).sqrt().round() as u32;

    while b * b != b2 {
        a = a + 1;
        b2 = a * a - n;
        b = (b2 as f64).sqrt().round() as u32;
    }

    a - b
}
