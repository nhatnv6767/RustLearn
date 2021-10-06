// helper.rs <same name>, <same directory>
mod helper;
mod group;

pub fn print_from_lib() {
    use helper::{print_from_helper, print_again};

    println!("Hello from lib");
    print_from_helper();
    group::g1::g1_hello();
}
