fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut the_largest = &list[0];

    for item in list {
        if item > the_largest {
            the_largest = item;
        }
    }

    the_largest
}

fn main() {
    let arr = vec![1, 2, 3, 1, 435, 45, 75, 4, 5, 345, 54];
    let the_largest_num = largest(&arr);

    println!("The largest number is {}", the_largest_num);

    let chars = vec!['d', 'f', 'g', 'a', 'w', 'z'];
    let the_largest_char = largest(&chars);

    println!("The largest char is {}", the_largest_char)
}
