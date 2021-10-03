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

// this is read as the type is something that implement the noise trait within our function body.
// So this may be an enumeration or struct
// We don't know -> As long as it implements the noise trait
// We then call the make_noise func that's defined up in the noise trait, which will then call
// the make noise function on the appropriate piece of data
fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

struct Person;
// for person structure, we're implementing the noise trait
// to implement a trait, use the impl keyword and instead of specifying the structure
// enumeration. You instead specify the name of the trait along with the keyword
// for and then the name of the structure or enumeration (for Person)
// Then within the implementation block, you'll list out all of the functions required for that specific
// trait.
// in the top, we only have one function -> so you only need to specify one function within the implementation body.
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
    // when call hello Person -> the make_noise function in hello will refer to the fn make_noise in the impl Noise for Person
    // => print out Hello
    hello(Person{});
    hello(Dog {});
}
