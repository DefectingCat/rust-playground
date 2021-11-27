fn main() {
    let arr = [1, 2, 3, 4, 5];

    for item in arr.iter() {
        println!("The value is {}", item)
    }

    for item in (1..4).rev() {
        println!("The tup value is {}", item)
    }
}
