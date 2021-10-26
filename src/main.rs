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
    let add = |a, b| a + b;
    let add = Box::new(add);
    println!("{}", math(2, 2, add));
}
