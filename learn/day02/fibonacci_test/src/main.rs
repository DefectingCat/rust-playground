fn main() {
    let res = fibonacci(3);
    println!("{}", res)
}

fn fibonacci(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut counter = n;

    while counter > 0 {
        counter -= 1;
        let (a, b) = (b, b + 1);
    }

    return b;
}
