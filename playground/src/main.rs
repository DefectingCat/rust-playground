fn main() {
    let v = vec![1, 2, 3, 4];
    let _: Vec<_> = v.into_iter().filter(|x| x > &2).collect();
}
