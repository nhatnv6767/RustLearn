enum Option<T> {
    Some(T),
    None,
}

struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name: "hahahah".to_owned(), qty: 4, },
        GroceryItem { name: "namamamnj".to_owned(), qty: 12, },
        GroceryItem { name: "LOLOLHKKK".to_owned(), qty: 7, },
    ];
    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }
    None
}

fn main() {

}