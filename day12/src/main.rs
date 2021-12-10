fn main() {
    let s1 = String::from("xfy");
    let result;

    {
        let s2 = String::from("Hello world!");
        result = longest(&s1[..], &s2[..]);
        println!("{}", result)
    }

    // println!("{}", result)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
