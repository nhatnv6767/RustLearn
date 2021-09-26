fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {

    let _add = |a: i32, b: i32| -> i32 {
        a + b
    };

    let ads = |a, b| a + b;
    let _sum = _add(1, 1);
    println!("{}",_sum);
}
