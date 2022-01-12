fn main() {
    let arr = [1, 2, 3];
    println!("{:p}", &arr);
    println!("{:p}", &arr[0]);
    println!("{:p}", &arr[1]);
    println!("{:p}", &arr[2]);

    let mut v = vec![String::from("1"), String::from("2"), String::from("3")];
    println!("{:p}", &v[1]);
    v[0] = String::new();
    println!("{:p}", &v[1]);
}
