fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let mut s = format!("{}-{}-{}", s1, s2, s3);
    s.push_str("!");
    println!("s is {}", s);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let mut s = String::from("12345");
    for c in s.bytes() {
        println!("byte is {}", c);
    }
    for c in s.chars() {
        println!("char is {}", c);
    }
    s.pop();
    println!("s is {}", s);

    let hello = "Здравствуйте";
    let answer = &hello[..2];
    println!("answer is {}", answer)
}
