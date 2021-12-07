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

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point::new(42, 3.14);
    let p2 = Point::new("Hello", "world");

    let p3 = p1.mixup(p2);

    println!("My point x is {}, and y is {}", p3.x(), p3.y())
}
