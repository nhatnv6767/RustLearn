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

    // the same meaning, with closures
    // it's similar to a function here we have our argument to our closure
    // num + 1 is the body of closure and it's going to automatically
    // return num + 1
    // |num| is the inner value of the option
    let plus_one = maybe_num().map(|num| num + 1);
}
