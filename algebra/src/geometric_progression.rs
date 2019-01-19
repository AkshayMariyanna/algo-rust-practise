/// Calculates GP - a^1 + a^2 + ... + a^n
///
/// O(log2(n))
///
/// ```
/// let a = 4;
/// let n = 3;
///
/// assert_eq!(algebra::geometric_progression::calc(a, n), 84);
/// ```
pub fn calc(mut a: u64, mut n: u64) -> u64 {
    let mut gp = 0;

    let mut gp_2_powers = a;
    while n != 0 {
        if n & 1 == 1 {
            gp = gp_2_powers + a * gp;
        }
        gp_2_powers = gp_2_powers + a * gp_2_powers;
        a *= a;
        n >>= 1;
    }

    gp
}

/// Calculates GP - a^1 + a^2 + ... + a^n modulus m
///
/// O(log2(n))
///
/// ```
/// let a = 4;
/// let n = 3;
/// let m = 17;
///
/// assert_eq!(algebra::geometric_progression::calc_mod(a, n, m), 16);
/// ```
pub fn calc_mod(mut a: u64, mut n: u64, m: u64) -> u64 {
    a = a % m;
    let mut gp = 0;

    let mut gp_2_powers = a;
    while n != 0 {
        if n & 1 == 1 {
            gp = (gp_2_powers + (a * gp) % m) % m;
        }
        gp_2_powers = (gp_2_powers + (a * gp_2_powers) % m) % m;
        a = (a * a) % m;
        n >>= 1;
    }

    gp
}
