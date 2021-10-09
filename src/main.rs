// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

// tuple structure, can be accessed with a .zero
#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor {
    fn new(color: Color) -> Self{
        Self(color)
    }
}

#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor {
    fn new(color: Color) -> Self{
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor {
    fn new(color: Color) -> Self{
        Self(color)
    }
}

// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function

fn print_shirt_color(color: Color) {
    println!("Shirt color = {:?}", color);
}


fn print_shoes_color(color: Color) {
    println!("Shoes color = {:?}", color);
}


fn print_pants_color(color: Color) {
    println!("Pants color = {:?}", color);
}

fn main() {
    let shirt_color = Color::Gray;
    let pants_color = Color::Blue;
    let shoes_color = Color::White;

    print_shirt_color(shoes_color);
    print_pants_color(pants_color);
    print_shoes_color(shirt_color);
}
