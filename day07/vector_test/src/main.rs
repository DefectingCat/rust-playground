fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello world!")),
    ];

    for item in &row {
        println!("Item: {:?}", item)
    }
}
