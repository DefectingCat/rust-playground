use std::fmt::{Debug, Display};
fn main() {
    let mut arr = vec![1, 92, 3, 90, 321, 26];

    check_arr(&arr);
    arr.push(42);
    check_arr(&arr);
}

fn check_arr<T: Display + Debug>(list: &[T]) {
    println!("The arr length is {:?} and arr is {:?}", list.len(), list)
}
