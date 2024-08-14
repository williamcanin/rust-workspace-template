/* If the application is not a console application, uncomment the line below. */
// #![cfg_attr(not(test), windows_subsystem = "windows")]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(target_os = "windows")]
#[link(name = "utils.dll", kind = "dylib")]
extern "C" {
  pub fn directory_user() -> *mut c_char;
  pub fn free_memory(ptr: *mut c_char);
  pub fn has_connection() -> bool;
  pub fn my_name(name: *const c_char) -> *mut c_char;
  pub fn fibonacci(n: u32) -> u64;
  pub fn is_palindrome(word: *mut c_char) -> bool;
}

#[cfg(target_os = "linux")]
#[link(name = "utils", kind = "dylib")]
extern "C" {
  pub fn directory_user() -> *mut c_char;
  pub fn free_memory(ptr: *mut c_char);
  pub fn has_connection() -> bool;
  pub fn my_name(name: *const c_char) -> *mut c_char;
  pub fn fibonacci(n: u32) -> u64;
  pub fn is_palindrome(s: &str) -> bool;
}

pub fn convert_string(result_ptr: *const c_char) -> String {
  unsafe {
    if result_ptr.is_null() {
      "<null>".to_string()
    } else {
      CStr::from_ptr(result_ptr)
        .to_str()
        .unwrap_or("<invalid UTF-8>")
        .to_string()
    }
  }
}

fn main() {
  // ================= Palindrome =================
  let word = "ovo";
  let c_string = CString::new(word).expect("CString::new failed");
  let c_ptr = c_string.into_raw(); // Converter para *mut c_char

  unsafe {
    match is_palindrome(c_ptr) {
      true => println!("The word \"{word}\" is a palindrome."),
      false => println!("The word \"{word}\" is not a palindrome."),
    }
  };

  // Convert pointer back to CString to avoid memory leak
  unsafe { free_memory(c_ptr) };

  // ================= Fibonacci =================
  let number = 10;

  unsafe {
    println!("Fibonacci of {number} is {}.", fibonacci(number));
  }

  unsafe {
    match has_connection() {
      true => println!("You are connected to the internet!"),
      false => println!("You are NOT connected to the internet!"),
    };
  }

  // ================= Home User =================

  // Use unsafe block to call the FFI function
  let current_dir = unsafe { directory_user() };

  // Convert the raw pointer to &str
  let current_dir_str = convert_string(current_dir);

  // Print the string
  println!("The current user directory is: {}", current_dir_str);

  // Free the memory allocated for the string
  unsafe { free_memory(current_dir) };

  // ================= My Name =================

  // Convert &str to CString and get a raw pointer
  let input = CString::new("Rust").unwrap();
  let input_ptr = input.as_ptr();

  // Call the FFI function
  let result_ptr = unsafe { my_name(input_ptr) };

  // Convert the result back to a string
  let result_str = convert_string(result_ptr);

  println!("Hello, {result_str}!");

  // Free the memory allocated for the result
  unsafe { free_memory(result_ptr) };
}
