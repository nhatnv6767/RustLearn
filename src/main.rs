use std::collections::HashMap;

fn main() {
    // Hashmap
    let mu = String::from("MU");
    let mc = String::from("MC");

    let mut scores = HashMap::new();

    scores.insert(mu, 10);
    scores.insert(mc, 9);

    let team_name = String::from("MU");
    // get nay la get lay theo index
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} {}", key, value)
    }

    println!("Score = {:?}", score);

}
