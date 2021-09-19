// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);

    let mut total_stock = 0;
    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            // it's similar to the print line macro, except instead of printing the line
            // on the console, it's just going to put it into a string instead
            // instead of printing it and just throwing everything away, we can save it
            // into a variable (saving it into the stock_count that we can use it later)
            format!("{:?}", qty)
        };
        println!("item = {}, stock = {}", item, stock_count);
    }

    println!("Total stock = {}", total_stock);
}
