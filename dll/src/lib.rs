// This dll can be executed with Windows "rundll32.exe" through the "main" function.
// To do this, after compiling the dll.dll, run the following in the terminal:
// >>> rundll32.exe path/to/dll.dll, main
mod utils;

use crate::utils::fs::create_file;

#[no_mangle]
pub extern fn sub(left: u64, right: u64) -> u64 {
  left - right
}

#[no_mangle]
// #[allow(improper_ctypes_definitions)]
pub unsafe extern fn main() {
  let filepath = r"C:\Users\admin\Desktop\file.txt".to_string();
  let content = "Hello".to_string();
  create_file(&filepath, &content);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = sub(7, 7);
    assert_eq!(result, 0);
  }
}
