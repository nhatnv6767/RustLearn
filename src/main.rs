fn main() {

    let mut v = vec![1, 2, 3, 4, 5, 6];

    for i in &mut v {
        *i += 10;
    }

    // in ra nhung gia tri trong vector
    for i in &v {
        println!("{}", i)
    }
}
