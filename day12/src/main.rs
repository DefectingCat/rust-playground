use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let novel = String::from("Call me xfy. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s1 = String::from("hello");
    let s2 = &s1;

    println!("{}", longest_with_an_announcement(&s1[..], &s2[..], "Test"))
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, an: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", an);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
