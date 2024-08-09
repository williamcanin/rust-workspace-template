mod tests;
mod utils;

pub fn add(left: u64, right: u64) -> u64 {
  left + right
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
