/* If the application is not a console application, uncomment the line below. */
// #![cfg_attr(not(test), windows_subsystem = "windows")]

use utils::{has_connection, my_name, directory_user, create_file, fibonacci, is_palindrome};

fn main() {
  create_file(&"file-rust.txt".to_string(), &"Hello!".to_string());

  let number = 10;
  println!("Fibonacci {number} is {}", fibonacci(number));

  let word = "ovo";
  match is_palindrome(word) {
    true => println!("The word \"{word}\" is a palindrome."),
    false => println!("The word \"{word}\" is not a palindrome."),
  };

  println!("The current user directory is: {}", directory_user());

  match has_connection() {
    true => println!("You are connected to the internet!"),
    false => println!("You are NOT connected to the internet!"),
  };

  let name = my_name("William");
  println!("Hello, {name}!");
}
