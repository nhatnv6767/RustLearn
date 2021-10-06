// helper.rs <same name>, <same directory>
mod helper;

pub fn print_from_lib() {
    println!("Hello from lib");
    helper::print_from_helper();
}
