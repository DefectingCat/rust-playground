fn main() {
    let arr = [1, 2, 3, 4];
    let _: Vec<_> = arr.iter().map(|item| println!("{}", item + 1)).collect();
}

fn closure_test() -> impl Fn(i32) -> i32 {
    let i = 42;
    move |j| j + i
}
