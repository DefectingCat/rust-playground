struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    println!("Hello, world!");
    let p1 = Point::new(42, 3.14);

    println!("The p1 x is {} and y is {}", p1.x(), p1.y())
}
