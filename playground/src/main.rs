use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;


fn main() {
    println!("{}", fibo(400));
}

fn fibo(n: usize) -> BigUint {
    let mut f0 = Zero::zero();
    let mut f1 = One::one();

    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }

    f0
}