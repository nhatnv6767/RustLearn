fn main() {
    let mut data = Some(3);
    while let Some(i) = data {
        println!("Loop");
        data = None;
    }

    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();
    while let Some(num) = number_iter.next() {
        println!("num = {:?}", num)
    }
    println!("Done");
}
