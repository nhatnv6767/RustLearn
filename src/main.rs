/*
    Implementing functionality with the impl keyword

    Requirement:
    Print the characteristics of a shipping box
    Must include dimensions, weight, and color

    Notes:
    Use a struct to encapsulate the box characteristics
    Use an enum for the box color
    Implement functionality on the box struct to create a new box
    Implement functionality on the box struct to print the characteristics
*/

//  Should create enum, and fn first, impl the 2nd and last is main()

//  Use an enum for the box color

enum Color {
    Brown,
    Red
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::Red => println!("Red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {}", self.width);
        println!("Height: {}", self.height);
        println!("Depth: {}", self.depth);
    }
}

//  Use a struct to encapsulate the box characteristics
//  Must include dimensions, weight, and color
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

//  Implement functionality on the box struct to create a new box
//  Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
}


fn main() {

}
