#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    fruit: String,
}

impl Breakfast {
    pub fn summer(toast: String) -> Breakfast {
        Breakfast {
            toast,
            fruit: String::from("Apple"),
        }
    }
    pub fn check_breakfast(&self) {
        println!("{:#?}", self);
    }
}
