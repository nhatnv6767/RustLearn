// The new types pattern leverages the rust type
// system in order to make your programs more reliable and easier to manage
//


// typle structure
#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    fn new(i: i32) -> Result<Self, String> {}
}

fn divide(a: i32, b: NeverZero) -> i32 {}

fn main() {}
