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

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    // inner just means we're taking the inner value of the bills [1]
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    // self : Bills structure was created somewhere else and we're just calling
    // a function implemented on
    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }
    // &Vec: we can return a borrowed vector in this case since it's already created in line [1]
    // and we can just borrow
    fn get_all(&self) -> Vec<Bill> {
        // the reason remove & that is we're creating the bills vector
        // so if we try to return a borrowed vector instead
        // the problem with that is the vector is created and this function owned that vector
        // and what's going to happen is this function is responsible for deleting all the data
        // that it created
        // which means it's going to immediately delete the vector
        let mut bills = vec![];
        for bill in self.inner.values() {
            bills.push(bill.clone());
        }
        return bills;
    }

    fn remove(&mut self, name: &str) -> bool {
        // go to docs: returns true if the option is a Some value
        // if we remove this we will get an optional value back
        self.inner.remove(name).is_some()
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    }
    return buffer.trim().to_owned();
}

fn get_bill_amount() -> f64 {
    println!("Amount:");
    loop {
        let input = get_input();
        // when you pass something, parsing just converts it from one format to another
        // convert string into a float
        // go to docs of RUST to read about parse to know the return value
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return amount,
            Err(_) => println!("Wrong type!. Please enter a number."),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    println!("Bill name");
    // get the bill name
    let name = get_input();
    // get the bill amount
    let amount = get_bill_amount();
    let bill = Bill { name, amount };
    bills.add(bill);
    println!("Bill added");
}

fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    let input = get_input();
    if bills.remove(&input) {
        println!("Removed");
    } else {
        println!("Bill not found")
    }
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    // can call show function only within the main_menu
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("");
        println!("Enter selection:");
    }

    let mut bills = Bills::new();
    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            _ => break,
        }
    }
}

fn main() {
    loop {
        main_menu(); // 1st
    }
}
