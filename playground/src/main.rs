fn main() {
    let mut s = String::new();

    let mut add_one = || s.push_str("string");

    add_one();

    println!("s: {}", s)
}
