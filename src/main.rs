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

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat discount is 2"),
        Discount::Flat(amount) => println!("flat discount of {}", amount),
        // ignore it, return nothing
        _ => (),
    }
}