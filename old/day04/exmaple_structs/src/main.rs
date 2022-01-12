#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.height > target.height
    }
    fn create_square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 42,
        height: 42,
    };
    let rect2 = Rectangle {
        width: 82,
        height: 82,
    };
    let rect3 = Rectangle {
        width: 22,
        height: 22,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The rect1 width > 0 ? {}", rect1.width());

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::create_square(42);
    println!("rect4: {:#?}", rect4)
}
