use std::collections::HashMap;

fn main() {
    // Hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Mu"), 10);
    scores.insert(String::from("Mu"), 300);

    println!("Scores = {:?}", scores);
}
