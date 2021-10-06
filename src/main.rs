use std::default;

// Default trait is used to create new structure
// in enumerations with a default value.

struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

// The reason you would want to implement default
// is because there are other create along with code
// within the standard library that will attempt to use default
// when applicable.
// It's a good idea to implement default for any structure or enumeration where
// it would make sense to have a default value
// because it only makes your code easier to use.
impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}
fn main() {}
