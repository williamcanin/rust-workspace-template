use std::fs;
use std::io::Write;

#[no_mangle]
// #[allow(improper_ctypes_definitions)]
pub extern fn create_file(
  filepath: &String,
  content: &String,
) {
  match fs::File::create(&filepath) {
    Ok(mut o) => o.write_all(content.as_bytes()).expect("Error writing in file"),
    Err(_) => panic!("error creating file: {:?}", &filepath),
  }
}
