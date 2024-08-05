/* If the application is not a console application, uncomment the line below. */
// #![cfg_attr(not(test), windows_subsystem = "windows")]

// mods
mod tests;
mod utils;

// imports
use lib::{userprofile, add, division};
use crate::utils::check::has_connection;

fn main() {
  let numerator = 20.0;
  let denominator = 4.0;
  match division(numerator, denominator) {
    Ok(result) => println!("{numerator} / {denominator} = {result}"),
    Err(err) => println!("Error: {err}")
  }

  match has_connection() {
    true => println!("You ARE connected to the internet."),
    false => println!("You are NOT connected to the internet.")
  };

  let doc_path = userprofile!("");
  println!("User root path: {}", doc_path.to_str().unwrap());

  let number1 = 12;
  let number2 = 8;
  let result = add(number1, number2);
  println!("{number1} + {number2} = {result}");
}
