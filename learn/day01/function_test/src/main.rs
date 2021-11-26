fn main() {
    println!("Hello, world!");
    my_function(42);

    let num = {
        let test = 40;
        test + 2
    };
    println!("Num: {:?}", num);

    let get_my_num = my_function(42);
    println!("My num: {}", get_my_num);

    println!("Plus one: {}", plus_one(41))
}

fn my_function(x: i32) -> i32 {
    println!("Hello, food!");
    println!("x is {}", x);

    x
}

fn plus_one(num: isize) -> isize {
    return num + 1;
}
