// modules

mod greet {
    fn hello() {
        println!("Hello");
    }
    fn goodbye() {
        println!("Goodbye");
    }
}
mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}
fn main() {
    use greet::hello;
    hello();
    greet::goodbye();
    math::add(1, 1);
}
