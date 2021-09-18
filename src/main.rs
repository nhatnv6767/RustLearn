/*
be showing how result can be utilized with functions that may possibly fail
a program that simulates a menu and the mnu will have multiple choices, the user
will be able to enter some text that corresponds to one of the choices.
if it fails, then the user will be notified the reason it fails internally
we're going to use an enumeration for each menu choice and we will utilize a function that
returns a results in order to transform the user input into the enumeration
*/

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

fn main() {
    println!("Hello, world!");
}
