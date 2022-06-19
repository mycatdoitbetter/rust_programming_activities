// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(expression_result: bool) {
  match expression_result {
    true => println!("its big!"),
    false => println!("its small!"),
  }
}

fn main() {
  let my_number = 123;

  let my_expression_result: bool = if my_number > 100 {
     true
  } else {
     false
  };

  print_message(my_expression_result);
}
