// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
  fn get_cartesian_coordinates() -> (i32, i32) {
    let cartesian_x = 3;
    let cartesian_y = 4;
    return (cartesian_x, cartesian_y);
  }

  let (_x, y) = get_cartesian_coordinates();

  if y > 5 {
    println!(">5")
  } else if y < 5 {
    println!("<5")
  } else {
    println!("equal")
  }
}
