// Mapping one value to another value
fn maybe_num() -> Option<i32> {

}

fn maybe_word() -> Option<String> {

}
fn main() {
    let plus_one = match maybe_num() {
        // mapping number to be the number + 1
        Some(num) => Some(num + 1),
        None => None,
    };
}
