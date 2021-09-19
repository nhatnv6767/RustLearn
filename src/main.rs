/*
- Collection that stores data as key-value pairs
    - Data is located using the "key"
    - The data is the "value"
- Similar to definitions in a dictionary
- Very fast to retrieve data using the key
 */
use std::collections::HashMap;

fn main() {
    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("Ed", 13);
    people.insert("Will", 14);
    people.insert("Cathy", 22);
    people.remove("Susan");

    match people.get("Ed") {
        Some(age) => println!("age = {:?}", age);
        None => println!("not found"),
    }
}
