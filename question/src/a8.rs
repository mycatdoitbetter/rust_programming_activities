// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
  Margerrita,
  Strawberry,
  Grape,
  Orange
}

struct Drink {
  flavor: Flavors,
  ounces: u8,
}

fn main() {
  let drink = Drink {
    flavor: Flavors::Orange,
    ounces: 21
  };

  match drink.flavor {
    Flavors::Grape => println!("grape flavor!"),
    Flavors::Strawberry => println!("strawberry flavor!"),
    Flavors::Orange => println!("orange flavor!"),
    Flavors::Margerrita => println!("margerrita flavor!"),
  }

  println!("with {} ounces", drink.ounces)
}
