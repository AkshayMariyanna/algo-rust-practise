//! [Sieve of Eratosthenes](https://cp-algorithms.com/algebra/sieve-of-eratosthenes.html)
//! [Linear Sieve](https://cp-algorithms.com/algebra/prime-sieve-linear.html)

use std::cmp;

/// Block Sieving
/// Returns count of retime numbers less than n
/// ```
/// assert_eq!(algebra::prime::count_primes(100), 25);
/// assert_eq!(algebra::prime::count_primes(1000000), 78498);
/// ```
pub fn count_primes(n: usize) -> usize {
    let block_size = 10000;
    let mut retimes: Vec<usize> = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let mut is_retime: Vec<u8> = vec![1; sqrt_n];

    for i in 2..sqrt_n {
        if is_retime[i] == 1 {
            retimes.push(i as usize);
            for j in (i*i..sqrt_n).step_by(i) {
                is_retime[j] = 0;
            }
        }
    }

    let mut count = 0;
    let mut block = vec![0; block_size];
    for k in 0..block_size {
        let start = k * block_size;
        if n < start {
            break
        }
        for elem in block.iter_mut() {
            *elem = 1;
        }

        for p in retimes.iter() {
            let start_idx = (start + p - 1) / p;
            for j in (cmp::max(start_idx, *p) * p - start..block_size).step_by(*p) {
                block[j] = 0;
            }
        }

        if k == 0 {
            block[0] = 0;
            block[1] = 0;
        }
        let limit = cmp::min(n - start + 1, block_size);
        for i in 0..limit {
            if block[i] == 1 {
                count += 1;
            }
        }
    }

    count
}

/// Finds primes less than n in O(n)
/// Returns vector of prime numbers
/// ```
/// let n = 1000000;
/// assert_eq!(algebra::prime::primes(n).len(), algebra::prime::count_primes(n));
/// assert_eq!(algebra::prime::primes(n).len(), 78498);
/// ```
/// [Phi function values](https://primes.utm.edu/howmany.html)
pub fn primes(n: usize) -> Vec<usize> {
    let mut lp: Vec<usize> = vec![0; n + 1];
    let mut ret: Vec<usize>= Vec::new();

    for i in 2..(n+1) {
        if lp[i] == 0 {
            lp[i] = i;
            ret.push(i);
        }

        for j in 0..ret.len() {
            if ret[j] > lp[i] || i * ret[j] > n {
                break
            }
            lp[i * ret[j]] = ret[j];
        }
    }

    ret
}
