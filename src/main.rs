// closures: Box
// The data within the box is surrounded by the <>
// The data type is a closure as specified Fn,
// we're passing in a simple style closure,
// we dont define any variable names for these arguments
// we only need to supply the data type and then we have the return value
// dyn: Dynamic:signifies to the compiler that this data could potentially be different things
// dyn: you might get multiple different kinds that are in this box instead of
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    // return operation was performed
    op(a, b)
}

fn main() {
    // create closure: accept a and b, then returns the a value plus the b value
    let add = |a, b| a + b;
    // saving this closure into a Box
    // do this in order to be able to call the closure from a function
    // because the function parameters need to have a known size
    // Closure sizes can veri from smal to large, and in order to call functions, we have to know exactly
    // what the size of the function argument is
    // when we place the closure into a box, the box is always guaranteed to be the same size as a pointer, which
    // is the size of a memory address
    let add = Box::new(add);
    println!("{}", math(2, 2, add));
}
