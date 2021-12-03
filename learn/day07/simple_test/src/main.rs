mod back_of_house;

use back_of_house::Breakfast;
use std::io;

fn main() {
    println!("Please input your toast:");

    let mut toast = String::new();
    io::stdin()
        .read_line(&mut toast)
        .expect("Failed to read line.");

    let meal = Breakfast::summer(toast.trim());

    println!("I'd a {} toast, please", meal.toast);

    meal.check_meal();
}
