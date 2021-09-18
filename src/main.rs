enum Option<T> {
    Some(T),
    None,
}

struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    let mark = Customer {
        age: Some(22),
        email: "mark@gmail.com".to_owned(),
    };
    let becky = Customer {
        age: None,
        email: "becky@gmail.com".to_owned(),
    };

    match becky.age {
        Some(age) => println!("Customer is {} years old", age),
        None => println!("Customer age not provided"),
    }
}