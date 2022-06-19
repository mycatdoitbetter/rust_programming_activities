// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


fn sum_two_numbers(first_number: i64, second_number: i64) -> i64 {
  first_number + second_number
}

fn main() {
  let result = sum_two_numbers(21, 32);

  println!("{:?}", result);
}
