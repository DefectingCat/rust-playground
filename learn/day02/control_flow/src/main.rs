use std::io;

fn main() {
    println!("Please input your age:");

    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line.");

    let age:u32 = age.trim().parse().expect("Please input a number");

    if age >= 18 {
        println!("Yeah!");
    } else {
        println!("No...");
    }
}
