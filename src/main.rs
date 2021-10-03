// the hash map would only be available to the main function
// because the module has its own version of different using statements
// if want to use hashmap with mod, put like example
use std::collections::HashMap;
// modules

mod greet {
    //
    use std::collections::HashMap;
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
