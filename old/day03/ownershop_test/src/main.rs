fn main() {
    let name = String::from("xfy");

    let new_name = takes_and_gives_back(name);
    println!("After changed: {}", new_name);
    // println!("{}", name) // 已经被移动
}

fn takes_and_gives_back(mut name: String) -> String {
    println!("Before change: {}", name);
    name = String::from("小肥羊");
    name
}
