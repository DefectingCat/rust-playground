fn main() {
    let list_of_nums = vec![1, 2, 3, 4, 5];
    let num_to_string = |i: &i32| i.to_string();

    let list_of_str: Vec<_> = list_of_nums.iter().map(num_to_string).collect();

    println!("{:?}", list_of_str);
}
