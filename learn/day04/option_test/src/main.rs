fn main() {
    let x: i8 = 5;
    let y: Option<i8> = None;

    println!("{}", y.unwrap_or(4))
}
