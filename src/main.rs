fn main() {

    let v = vec![1, 2, 3, 4, 5, 6];

    match v.get(20) {
        Some(four) => println!("This is four element = {}", four),
        None => println!("This is not a four element"),
        // se in ra dong None => vi no ko phai element thu 4
    }


}
