fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // 返回对应值的引用，可以直接对其进行修改
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:?}", map);
}
