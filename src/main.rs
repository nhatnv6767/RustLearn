/*
be showing how result can be utilized with functions that may possibly fail
a program that simulates a menu and the mnu will have multiple choices, the user
will be able to enter some text that corresponds to one of the choices.
if it fails, then the user will be notified the reason it fails internally
we're going to use an enumeration for each menu choice and we will utilize a function that
returns a results in order to transform the user input into the enumeration
*/

// to allow to print out these variants on the terminal without having to manually match on each one
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}
// inside Result is Ok and Err
fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice not found".to_owned()),
    }
}
// borrow MenuChoice
fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    // the ? will automatically perform a match operation and happen is if the result is an OK
    // then that inner data will get placed into the choice
    // If it's the error var, the error is going to get automatically returned as the error
    // shorter than use match block to check each one
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}


fn main() {
    let choice = pick_choice("end");
    println!("choice value = {:?}", choice);
}
