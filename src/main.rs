// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    // inner just means we're taking the inner value of the bills [1]
    inner: Vec<Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }
    // self : Bills structure was created somewhere else and we're just calling
    // a function implemented on
    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }
    // &Vec: we can return a borrowed vector in this case since it's already created in line [1]
    // and we can just borrow
    fn get_all(&self) -> &Vec<Bill> {
        &self.inner
    }
}

fn main() {}
