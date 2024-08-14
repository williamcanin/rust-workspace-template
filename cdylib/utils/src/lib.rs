mod macros;
mod tests;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn fibonacci(n: u32) -> u64 {
  if n <= 1 {
    n as u64
  } else {
    fibonacci(n - 1) + fibonacci(n - 2)
  }
}

#[no_mangle]
pub extern "C" fn is_palindrome(word: *mut c_char) -> bool {
  let c_str = unsafe {
    assert!(!word.is_null());
    CStr::from_ptr(word)
  };

  let r_str = match c_str.to_str() {
    Ok(string) => string,
    Err(_) => return false, // If the conversion fails, it is not a palindrome.
  };

  let cleaned: String = r_str
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect();
  cleaned == cleaned.chars().rev().collect::<String>()
}

#[no_mangle]
pub extern "C" fn directory_user() -> *mut c_char {
  // Get the user's home directory
  let user_home = userprofile!("").to_str().unwrap().to_string();

  // Convert the string to CString
  let c_str = CString::new(user_home).unwrap();

  // Convert CString to a raw pointer
  c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn free_memory(ptr: *mut c_char) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    // Convert the raw pointer back into a CString and drop it
    CString::from_raw(ptr).to_str().unwrap();
  }
}

#[no_mangle]
pub extern "C" fn my_name(name: *const c_char) -> *mut c_char {
  if name.is_null() {
    return std::ptr::null_mut();
  }

  let c_str = unsafe { CStr::from_ptr(name) };
  let name_str = match c_str.to_str() {
    Ok(s) => s,
    Err(_) => return std::ptr::null_mut(),
  };

  let first_letter = name_str[..1].to_string().to_uppercase();
  let rest = name_str[1..].to_string();
  let result = format!("{}{}", first_letter, rest);

  CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn has_connection() -> bool {
  let result = reqwest::blocking::get("https://google.com");
  match result {
    Ok(_) => true,
    Err(_) => false,
  }
}
