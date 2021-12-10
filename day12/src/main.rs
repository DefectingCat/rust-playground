struct SomeThing<'a> {
    part: &'a str,
}

impl<'a> SomeThing<'a> {
    fn new(part: &'a str) -> Self {
        SomeThing { part }
    }

    fn show_something(&self) -> &str {
        self.part
    }
    fn show_something_and_anohter(&self, another: &str) -> String {
        format!("{} and {}", self.part, another)
    }
}

fn main() {
    let my_something = SomeThing::new("xfy");
    println!("{}", my_something.show_something());

    let s1;
    {
        let s2 = String::from("hello");
        s1 = &s2;
        println!("{}", my_something.show_something_and_anohter(&s1[..]));
    }
}

// I love Rust.
