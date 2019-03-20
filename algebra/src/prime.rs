//! Sieve of Eratosthenes
//! https://cp-algorithms.com/algebra/sieve-of-eratosthenes.html

use std::cmp;

/// Block Sieving
/// Returns count of prime numbers less than n
/// ```
/// assert_eq!(algebra::count_primes(100), 25);
/// assert_eq!(algebra::count_primes(1000000), 78498);
/// ```
pub fn count_primes(n: usize) -> usize {
    let block_size = 10000;
    let mut primes: Vec<usize> = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let mut is_prime: Vec<u8> = vec![1; sqrt_n];

    for i in 2..sqrt_n {
        if is_prime[i] == 1 {
            primes.push(i as usize);
            for j in (i*i..sqrt_n).step_by(i) {
                is_prime[j] = 0;
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

        for p in primes.iter() {
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
