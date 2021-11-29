fn main() {
    let mut s1 = String::from("Hello world");

    let r1 = &mut s1;
    // let r2 = &mut s1; // cannot borrow `s1` as mutable more than once at a time
    println!("{}", r1)
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("string: &str");
    s.len()
}
