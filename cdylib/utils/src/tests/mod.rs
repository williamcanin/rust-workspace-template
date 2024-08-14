#[cfg(test)]
mod tests {
  use crate::has_connection;

  #[test]
  fn test_connection() {
    match has_connection() {
      true => assert!(true),
      false => assert!(false),
    }
  }
}
