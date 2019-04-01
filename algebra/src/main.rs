use algebra::GNumber;
use num::BigInt;

fn main() {
    println!("{}", (&GNumber::new(&BigInt::from(-1000687000001_i64))).to_bigint());
}
