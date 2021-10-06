// helper.rs <same name>, <same directory>
mod helper;

pub fn print_from_lib() {
    use helper::{print_from_helper, print_again};

    println!("Hello from lib");
    print_from_helper();
}
