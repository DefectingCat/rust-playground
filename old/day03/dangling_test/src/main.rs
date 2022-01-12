fn main() {
    let something = no_dangle();
    println!("{}", something)
}

fn no_dangle() -> String {
    let s = String::from("s: &str");
    s
}
