fn main() {
    let x: i8 = 5;
    let y: Option<i8> = None;

    println!("{}", y.unwrap_or(4));

    let mut name = String::from("Hello");
    change_name(&mut name)
}

fn change_name(name: &mut String) {
    println!("name is {}", name);
    *name = String::from("s: &str");
    println!("name is {}", name);
}
