// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

// * Use an enum to store the possible power states
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

// * Use a match expression to convert the user input into the power state enum
impl PowerState {
    // use Option because the user can type in anything they want
    // Option (Some and None)
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();
        match state {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ = None
        }
    }
}

// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
fn print_power_action(state: PowerState) {
    // it uses modules from the standard library and you can also utilize it
    // for using different items that are available in other parts of code
    // take everything in PowerState and use in this function
    use PowerState::*;
    match state {
        Off => println!("Turning off"),
        Sleep => println!("Sleeping"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }

}

fn main() {
    let mut buffer = String::new();
    // dont need to use ? after
    let user_input_status = io::stdin().read_line((&mut buffer));
    // ok here is defined on results and the read_line returns a result
    if user_input_status.is_ok() {
        // the buffer in after main is a owned string and the powerstate new function
        // require a borrwed
        match PowerState::new(&buffer) {
            Some(state) => print_power_action(state),
            None => println!("Invalid power state"),
        }
    } else {
        println!("Error reading input");
    }
}
