#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let rect1 = Rectangle {
        width: 42,
        height: 42,
    };
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
