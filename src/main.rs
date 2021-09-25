fn main() {
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("user = {:?}", user),
        None => println!("No user"),
    }
}
