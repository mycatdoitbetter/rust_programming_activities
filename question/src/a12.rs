// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics ✅
// * Use an enum for the box color ✅
// * Implement functionality on the box struct to create a new box ✅
// * Implement functionality on the box struct to print the characteristics ✅

enum Colors {
  Blue,
  Red,
}

impl Colors {
  fn print(&self) {
    match self {
      Colors::Blue => println!("its blue"),
      Colors::Red => println!("its red"),
    }
  }
}

struct Dimensions {
  width: f64,
  height: f64,
  deepth: f64,
  weight: f64,
}

impl Dimensions {
  fn print(&self) {
    println!(
      "width: {}, height: {}, deepth: {}, weight: {}",
      self.width, self.height, self.deepth, self.weight
    )
  }
}

struct BoxDescription {
  dimensions: Dimensions,
  color: Colors,
}

impl BoxDescription {
  fn new(dimensions: Dimensions, color: Colors) -> BoxDescription {
     BoxDescription {
      dimensions,
      color,
    }
  }
}

fn main() {
  let dimensions = Dimensions {
    width: 123.0,
    height:  123.0,
    deepth:  123.0,
    weight:  123.0,
  };


  let my_box = BoxDescription::new(dimensions, Colors::Blue);
  my_box.color.print();
  my_box.dimensions.print();
}
