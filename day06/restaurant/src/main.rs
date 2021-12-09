pub use crate::back_of_house;

fn main() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    meal.get_self();
    println!("I'd like {} toast, please", meal.toast);
}
