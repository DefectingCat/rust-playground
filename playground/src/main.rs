fn main() {
    let a = 2;
    let b = 3;
    println!("{}", closure_test()(a));
    println!("{}", closure_test()(b));
}

fn closure_test() -> impl Fn(i32) -> i32 {
    let i = 42;
    move |j| j + i
}
