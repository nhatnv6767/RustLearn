fn main() {
    // Vectors bang cach su dung enum

    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SheetCell::Int(5),
        SheetCell::Float(10.12),
        SheetCell::Text(String::from("Blue")),
    ];

    match &row[1] {
        &SheetCell::Float(i) => println!("{}", i),
        _ => println!("This is not float"),
    }
}
