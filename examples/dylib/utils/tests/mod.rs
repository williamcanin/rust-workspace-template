#[cfg(test)]
mod tests {
  use utils::{has_connection, my_name};
  #[test]
  fn test_connection() {
    match has_connection() {
      true => assert!(true),
      false => assert!(false),
    }
  }

  #[test]
  fn test_my_name() {
    assert_eq!(my_name("Rust"), "Rust");
  }
}
