fn main() {
    let mut data = Some(3);
    while let Some(i) = data {
        println!("Loop");
        data = None;
    }
    println!("Done");
}
