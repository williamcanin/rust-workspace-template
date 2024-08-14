use std::io::Write;

mod macros;

pub fn create_file(
  filepath: &String,
  content: &String,
) {
  match std::fs::File::create(&filepath) {
    Ok(mut o) => {
      o.write_all(content.as_bytes()).expect("Error writing in file");
      println!("File \"{filepath}\" created!");
    },
    Err(_) => panic!("error creating file: {:?}", &filepath),
  }
}

pub fn directory_user() -> String {
  let user_home = userprofile!("");
  user_home.to_str().unwrap().to_string()
}

pub fn my_name(name: &str) -> String {
  let first_letter = name[..1].to_string().to_uppercase();
  let rest = name[1..].to_string();
  format!("{}{}", first_letter, rest)
}

pub fn has_connection() -> bool {
  let result = reqwest::blocking::get("https://google.com");
  match result {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn division(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
  match denominator {
    0.0 => Err("Division by zero is not allowed"),
    _ => Ok(numerator / denominator),
  }
}

pub fn fibonacci(n: u32) -> u64 {
  if n <= 1 {
    n as u64
  } else {
    fibonacci(n - 1) + fibonacci(n - 2)
  }
}

pub fn is_palindrome(s: &str) -> bool {
  let cleaned: String = s
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect();
  cleaned == cleaned.chars().rev().collect::<String>()
}
