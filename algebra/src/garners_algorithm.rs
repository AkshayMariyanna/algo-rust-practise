//! [Chinese Remainder Theorem](https://cp-algorithms.com/algebra/chinese-remainder-theorem.html)
use num::{BigInt, Zero, One, cast::ToPrimitive};
use std::ops::{Add, Sub, Mul};

use crate::primality::deterministic_millerrabin;
use crate::modular_inverse::modular_inverse_extended_gcd;

const SZ: usize = 100;


pub struct GNumber {
    a: [i64; SZ],
}

impl GNumber {
    pub fn default() -> GNumber {
        GNumber { a: [0; SZ] }
    }
    pub fn new(n: &BigInt) -> GNumber {
        let (pr, _) = primes_and_inverses();
        let mut a = [0; SZ];
        for i in 0..SZ {
            a[i] = (n % pr[i]).to_i64().unwrap();
        }

        GNumber {
            a
        }
    }

    pub fn to_bigint(&self) -> BigInt {
        let (pr, r) = primes_and_inverses();
        let mut result = BigInt::zero();
        let mut mult = BigInt::one();
        let mut x = [0; SZ];
        for i in 0..SZ {
            x[i] = self.a[i];
            for j in 0..i {
                let cur = (x[i] - x[j]) * r[j][i];
                x[i] = (cur % pr[i] + pr[i]) % pr[i];
            }

            result += &mult * x[i];
            mult *= pr[i];
        }

        // if result >= (&mult >> 1) {
        //     result -= mult;
        // }

        result
    }
}

impl<'a, 'b> Add<&'b GNumber> for &'a GNumber {
    type Output = GNumber;
    fn add(self, n: &'b GNumber) -> GNumber {
        let (pr, _) = primes_and_inverses();
        let mut ret = GNumber::default();
        for i in 0..SZ {
            ret.a[i] = (self.a[i] + n.a[i]) % pr[i];
        }

        ret
    }
}

impl<'a, 'b> Sub<&'b GNumber> for &'a GNumber {
    type Output = GNumber;
    fn sub(self, n: &'b GNumber) -> GNumber {
        let (pr, _) = primes_and_inverses();
        let mut ret = GNumber::default();
        for i in 0..SZ {
            ret.a[i] = (self.a[i] - n.a[i] + pr[i]) % pr[i];
        }
        ret
    }
}

impl<'a, 'b> Mul<&'b GNumber> for &'a GNumber {
    type Output = GNumber;
    fn mul(self, n: &'b GNumber) -> GNumber {
        let (pr, _) = primes_and_inverses();
        let mut ret = GNumber::default();
        for i in 0..SZ {
            ret.a[i] = (self.a[i] * n.a[i]) % pr[i];
        }
        ret
    }
}

fn primes_and_inverses() ->(&'static [i64; SZ], &'static [[i64; SZ]; SZ]) {
    use std::sync::Once;
    static INIT: Once = Once::new();

    static mut PR: [i64; SZ] = [0; SZ];
    static mut R: [[i64; SZ]; SZ] = [[0; SZ]; SZ];

    INIT.call_once(|| {
        let mut x = 1000000000_i64;
        let mut i = 0;
        while i < SZ {
            if deterministic_millerrabin(x as u64) {
                unsafe {
                    PR[i] = x;
                }
                i += 1;
            }

            x += 1;
        }

        for j in 0..SZ {
            for k in j+1..SZ {
                unsafe {
                    R[j][k] = modular_inverse_extended_gcd(PR[j], PR[k]).unwrap();
                }
            }
        }
    });

    unsafe {
        (&PR, &R)
    }
}
