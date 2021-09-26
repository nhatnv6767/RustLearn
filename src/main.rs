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
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }

    fn get_all(&self) -> Vec<Bill> {
        let mut bills = vec![];
        for bill in self.inner.values() {
            bills.push(bill.clone());
        }
        return bills;
    }

    fn remove(&mut self, name: &str) -> bool {
        return self.inner.remove(name).is_some();
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    eprint!("Amount (Blank if you dont want to add new bill):");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => eprint!("Wrong type..Please enter a number!\nAmount: "),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    eprint!("Bill name (Blank if you dont want to add new bill): ");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    let bill = Bill { name, amount };
    bills.add(bill);
    println!("Bill added");
}

fn check_empty_list(bills: &Bills) -> bool {
    let list_bills = bills.get_all();
    if list_bills.is_empty() == false {
        return true;
    } else {
        return false;
    }
}

fn remove_bill_menu(bills: &mut Bills) {
    view_bills_menu(bills);
    eprint!("Enter bill name to remove (Blank if you dont want to add new bill): ");
    let input = match get_input() {
        Some(input) => input,
        None => return,
    };
    if bills.remove(&input) {
        println!("Removed");
    } else {
        println!("Bill not found");
    }
}

fn update_bill_menu(bills: &mut Bills) {
    view_bills_menu(bills);

    eprint!("Enter bill name to update (Blank if you dont want to add new bill): ");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };

    if bills.update(&name, amount) {
        println!("Updated");
    } else {
        println!("Bill not found");
    }
}

fn view_bills_menu(bills: &Bills) {
    let list_bills = bills.get_all();
    if list_bills.is_empty() == false {
        for bill in list_bills {
            println!("{:?}", bill);
        }
    } else {
        println!("Have no list");
    }
}

fn main_menu() {
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("");
        eprint!("Enter selection: ");
    }
    let mut bills = Bills::new();
    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => update_bill_menu(&mut bills),
            _ => break,
        }
    }
}

fn main() {
    loop {
        main_menu();
    }
}
