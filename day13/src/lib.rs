pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn can_hold(&self, other: Self) -> bool {
        self.width * self.height > other.width * other.height
    }
}

mod tests {
    use super::Rectangle;

    #[test]
    fn hold_test() {
        let larger = Rectangle::new(42, 42);
        let samller = Rectangle::new(31, 31);

        assert!(larger.can_hold(samller))
    }

    #[test]
    fn failed_test() {
        let larger = Rectangle::new(42, 42);
        let samller = Rectangle::new(31, 31);

        assert!(samller.can_hold(larger))
    }
}
