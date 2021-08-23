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

// should create enum, and fn first, impl the 2nd and last is main()

// Use an enum for the box color

enum Color {
    Brown,
    Red
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

// Use a struct to encapsulate the box characteristics
// Must include dimensions, weight, and color
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

fn main() {

}
