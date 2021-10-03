// Range is simply an automated way to create a range of
// values.
fn main() {
    let range = 1..=3;
    // not include the last value
    let range = 1..4;
    for num in 1..4 {
        println!("{:?}", num)
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch)
    }
}
