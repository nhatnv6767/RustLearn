// fundamentail: Trail
// . A way to specify that some functionality exists
// . Used to standardize functionality across multiple different types
// - Standardization permits functiona to operate on multiple different types
// - Code deduplication
// you would only need to write one function that operates upon a trait, and
// then can call that function with multiple different data types as long as they
// all implement that trait
trait Noise {
    fn make_noise(&self);
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("Hello")
    }
}
struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("woof")
    }
}
fn main() {
    hello(Person{});
    hello(Dog {});
}
