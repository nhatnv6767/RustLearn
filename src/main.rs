struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for item in receipt {
        // in the 2nd line is String data type as the name, so this is an owned string
        // however, print_name function requires a borrowed string slice.
        // so in here, can't just send in the item name because that is a string, so instead we simply need to borrow the name filed "&item.name"
        // that mean we were borrowing this filed here, the name field and the 2nd line is going to create a string slice from that (2nd) string
        print_name(&item.name);
        println!("count: {}", item.count);
    }
}