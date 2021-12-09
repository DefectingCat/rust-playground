mod back_of_house;
use std::io;

pub use back_of_house::Breakfast;

fn main() {
    println!("Please input your toase:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let meal = Breakfast::summer(input);
    meal.check_breakfast();
    println!("I'd like a {} toast, please", meal.toast)
}
