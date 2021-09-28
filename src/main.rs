use std::collections::HashMap;

fn main() {
    // Hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Mu"), 10);
    scores.insert(String::from("Mu"), 300);

    // y nghia cua entry: Neu chua co gia tri se insert,
    // con nhu neu da co gia tri thi se bo qua
    scores.entry(String::from("MC")).or_insert(10);
    scores.entry(String::from("MC")).or_insert(200);
    println!("Scores = {:?}", scores);
}
