// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

// * Use a struct to store at least the age of a customer
struct Customer {
    age: i32
}

// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
fn try_purchase(customer: &Customer) -> Result<(), String> {
    // * Restricted purchases require that the age of the customer
    //   is at least 21
    if customer.age < 21 {
        // * The Err variant should detail the reason why they cannot make a purchase
        Err("Customer must be at least 21 years old".to_string())
    } else {
        Ok(())
    }
}


fn main() {
    println!("Hello, world!");
}
