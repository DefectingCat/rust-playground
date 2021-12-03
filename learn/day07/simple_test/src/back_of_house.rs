pub struct Breakfast {
    pub toast: String,
    fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            fruit: String::from("Apple"),
        }
    }
}

impl Breakfast {
    pub fn check_meal(&self) {
        println!(
            "Here is your {} toast and {} fruit.",
            self.toast, self.fruit
        )
    }
}
