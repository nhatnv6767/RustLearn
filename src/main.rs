enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 6;
    match n {
        3 => println!("three"),
        other => println!("number: {}", other),
    }
}