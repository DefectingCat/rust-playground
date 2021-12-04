use std::io;

fn main() {
    println!("Please input number of n:");

    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line.");
        let n: u128 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please input a number: ",);
                continue;
            }
        };

        let result = fibonacci(n);
        println!("Result is {}", result)
    }
}

fn fibonacci(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let mut fibo = (1, 1);

    for _ in 2..n {
        fibo = (fibo.1, fibo.0 + fibo.1);
    }
    fibo.1
}
