mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }
    impl Breakfast {
        pub fn get_self(&self) {
            println!("{:#?}", &self);
        }
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("Apple"),
            }
        }
    }
}
