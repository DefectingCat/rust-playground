fn main() {
    let mut name1 = Some(String::from("hello"));
    modify_some(&mut name1);
    println!("{:?}", name1);
}

fn modify_some(name: &mut Option<String>) {
    match name {
        None => None,
        Some(value) => Some(*value = String::from("world")),
    };
}
