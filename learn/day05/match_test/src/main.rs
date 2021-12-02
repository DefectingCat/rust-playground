fn main() {
    let num: Option<i8> = Some(3);
    let num2 = plus_one(num);

    println!("num is {:?}", num);
    println!("num2 is {:?}", num2);
}

fn plus_one(num: Option<i8>) -> Option<i8> {
    match num {
        None => None,
        Some(value) => Some(value + 1),
    }
}
