fn main() {
    let a = String::from("xfy");
    let b = 2;

    {
        println!("{:p} {:p}", &a, &b);
        let a = 3;
        println!("a : {}", a);
    }

    println!("a : {}", a);
}
