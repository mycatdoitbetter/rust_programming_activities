// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
  Green,
  Red, 
  Blue
}

fn show_color (my_color: Color) {

  match my_color {
    Color::Green => println!("green"),
    Color::Red => println!("red"),
    Color::Blue => println!("blue"),
  }
}

fn main() {
  // * Use an enum with caolor names as variants
  show_color(Color::Blue)
 
}
