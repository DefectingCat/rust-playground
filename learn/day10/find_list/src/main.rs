fn main() {
    let number_list = vec![1, 2, 6, 43, 5, 7, 5, 23, 3, 6, 4568, 456];

    println!("The largest is {}", largest(&number_list))
}

fn largest(list: &[i32]) -> i32 {
    let mut the_largest = list[0];

    for &item in list {
        if item > the_largest {
            the_largest = item
        }
    }
    the_largest
}
