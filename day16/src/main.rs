use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let my_name = MyBox::new(String::from("xfy"));
    hello(&my_name);
}

fn hello(name: &str) -> String {
    println!("Hello, {}!", name);
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_deref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_box() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_hello() {
        let my_name = MyBox::new(String::from("xfy"));

        assert_eq!("Hello, xfy!", hello(&my_name));
    }
}
