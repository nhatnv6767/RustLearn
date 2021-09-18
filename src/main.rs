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
// explain: how Rust is able to guarantee that your programs run properly because
// MenuChoice data is wrapped within a result. And if you try to use it without getting the inner value, you get a compiler error
fn main() {
    let choice: Result<MenuChoice, _> = get_choice("mainmenu");
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {}", e),
    }
}
