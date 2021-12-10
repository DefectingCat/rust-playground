struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

fn main() {
    let me = Person::new("xfy");

    println!("Hello {}", me.name)
}
