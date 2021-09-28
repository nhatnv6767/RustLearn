fn main() {

    let v = vec![1, 2, 3, 4, 5, 6];

    // in ra nhung gia tri trong vector
    for i in &v {
        println!("{}", i)
    }
}
