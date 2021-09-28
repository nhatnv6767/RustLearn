use std::collections::HashMap;

fn main() {
    // Hashmap
    let mu = String::from("MU");
    let mc = String::from("MC");

    let mut scores = HashMap::new();

    scores.insert(mu, 10);
    scores.insert(mc, 9);

    for (key, value) in &scores {
        println!("{} {}", key, value)
    }


}
