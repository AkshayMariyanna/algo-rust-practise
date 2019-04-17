//! [Linear Diophantine Equations](https://cp-algorithms.com/algebra/linear-diophantine-equation.html)
pub struct Solution(pub i64, pub i64);

mod helper {
    use super::Solution;
    pub fn gcd(a: i64, b: i64) -> (Solution, i64) {
        if a == 0 {
            (Solution(0, 1), b)
        } else {
            let (Solution(x, y), g) = gcd(b % a, a);
            (Solution(y - (b / a) * x, x), g)
        }
    }

    pub fn shift_solution(Solution(x, y): Solution, a_g: i64, b_g: i64, offset: i64) -> Solution {
        Solution(x + offset * b_g, y - offset * a_g)
    }
}

/// Finds an integer solution (x, y) to ax + by = c
/// Also returns GCD as the third element in the tuple
/// 
/// Returns None if no integer solution possible
/// ```
/// let a = 4;
/// let b = 6;
/// let c = 24;
/// 
/// if let Some((s, _)) = algebra::linear_diophantine::find_one_solution(a, b, c) {
///     assert_eq!(s.0 * a + s.1 * b, c);
/// } else {
///     panic!("This shouldn't be None");
/// }
/// 
/// let a = 4;
/// let b = 6;
/// let c = 23;
/// 
/// if let None = algebra::linear_diophantine::find_one_solution(a, b, c) {
///     // Do something
/// } else {
///     panic!("This should be None");
/// }
/// ```
pub fn find_one_solution(a: i64, b: i64, c: i64) -> Option<(Solution, i64)> {
    let (Solution(x, y), g) = helper::gcd(a.abs(), b.abs());
    if c % g != 0 {
        None
    } else {
        let mult = c / g;
        let x = x * if a < 0 { -mult } else { mult };
        let y = y * if b < 0 { -mult } else { mult };

        Some((Solution(x, y), g))
    }
}

/// Finds count of integer solutions (x, y) to ax + by = c 
/// with x bound to [xmin, xmax] and y bound to [ymin, ymax]
///
/// ```
/// let a = 4;
/// let b = 6;
/// let c = 10;
/// 
/// let xmin = 0;
/// let xmax = 4;
/// let ymin = -1;
/// let ymax = 1;
/// 
/// assert_eq!(algebra::linear_diophantine::number_of_solutions(a, b, c, xmin, xmax, ymin, ymax), 2);
/// ```
pub fn number_of_solutions(a: i64, b: i64, c: i64, xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> i64 {
    match find_one_solution(a, b, c) {
        None => 0,
        Some((mut s, g)) => {
            let a_g = a / g;
            let b_g = b / g;

            let sign_a = if a < 0 { -1 } else { 1 };
            let sign_b = if b < 0 { -1 } else { 1 };

            let offset = (xmin - s.0) / b_g;
            s = helper::shift_solution(s, a_g, b_g, offset);
            if s.0 < xmin {
                s = helper::shift_solution(s, a_g, b_g, sign_b);
            }
            if s.0 > xmax  {
                return 0
            }
            let lx1 = s.0;

            let offset = (xmax - s.0) / b_g;
            s = helper::shift_solution(s, a_g, b_g, offset);
            if s.0 > xmax {
                s = helper::shift_solution(s, a_g, b_g, -sign_b);
            }
            let rx1 = s.0;

            let offset = -(ymin - s.1) / a_g;
            s = helper::shift_solution(s, a_g, b_g, offset);
            if s.1 < ymin {
                s = helper::shift_solution(s, a_g, b_g, -sign_a);
            }
            if s.1 > ymax {
                return 0
            }
            let mut lx2 = s.0;

            let offset = -(ymax - s.1) / a_g;
            s = helper::shift_solution(s, a_g, b_g, offset);
            if s.1 > ymax {
                s = helper::shift_solution(s, a_g, b_g, sign_a);
            }
            let mut rx2 = s.0;

            if lx2 > rx2 {
                let temp = lx2;
                lx2 = rx2;
                rx2 = temp;
            }

            let lx = if lx1 < lx2 { lx2 } else { lx1 };
            let rx = if rx1 < rx2 { rx1 } else { rx2 };

            if lx > rx { 0 } else { (rx - lx) / b_g.abs() + 1 }
        }
    }
}
