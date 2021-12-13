pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value)
        }

        Self { value }
    }
}

mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test() {
        Guess::new(101);
    }
}
