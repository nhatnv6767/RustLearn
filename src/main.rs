// Iterators are a simple way to traverse and manipulate data structure
//
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut plus_one = vec![];
    for num in numbers {
        plus_one.push(num + 1);
    }
}
