// Mapping one value to another value
fn maybe_num() -> Option<i32> {

}

fn maybe_word() -> Option<String> {

}
fn main() {
    // transformed a string into an integer
    // the word length will be an optional integer
    let word_length = maybe_word().map(|word| word.len());
}
