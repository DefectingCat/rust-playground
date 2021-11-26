fn main() {
    let x = 40;
    let x = x + 1;

    {
        let x = 42;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
