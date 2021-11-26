use std::io;

fn main() {
    println!("Please input a number: ");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let arr_len = months.len();

    if index - 1 <= arr_len {
        println!("The month is {}", months[index - 1]);
    } else {
        println!("Out of range!");
    }
}
