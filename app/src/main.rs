/* If the application is not a console application, uncomment the line below. */
// #![cfg_attr(not(test), windows_subsystem = "windows")]

// mods
mod tests;
mod utils;

// imports
use crate::utils::check::has_connection;
use lib::{userprofile, add, division};

#[cfg(target_os = "windows")]
#[link(name = "dll.dll", kind = "dylib")]
extern {
  #[allow(improper_ctypes)]
  pub fn sub(left: u64, right: u64) -> u64;
  #[allow(improper_ctypes)]
  pub fn create_file(filepath: &String, content: &String);
}



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

  let number1 = 10;
  let number2 = 4;
  let result = unsafe { sub(number1, number2) };
  println!("{number1} - {number2} = {result}");

  let filepath = r"C:\Users\admin\Desktop\file.txt".to_string();
  let content = "Hello".to_string();

  #[cfg(target_os = "windows")]
  unsafe { create_file(&filepath, &content); }
}
