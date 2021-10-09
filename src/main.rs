// The new types pattern leverages the rust type
// system in order to make your programs more reliable and easier to manage
//


// typle structure
#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    pub fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("cannot be zero".to_string())
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    // if recall from the lecture on typles to access the fields of the tuple
    // just provide the index
    // this is a typle structure and we only have one item and the index starts at zero
    let b = b.0;
    return a / b;
}

fn main() {
    match NeverZero::new(5) {
        Ok(nz) => println!("{:?}", divide(10, nz)),
        Err(e) => println!("{:?}", e),
    }
}
