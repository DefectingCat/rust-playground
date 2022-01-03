pub fn fibonacci(n: u32) -> u32 {
    if n <= 3 {
        return n;
    }

    let mut fibo = (2, 3);

    for _ in 3..n {
        fibo = (fibo.1, fibo.0 + fibo.1);
    }

    fibo.1
}
