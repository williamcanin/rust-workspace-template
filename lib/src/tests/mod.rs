#[cfg(test)]
mod tests {
  use crate::{add, fibonacci, is_palindrome, division};

  #[test]
  fn t_add() {
    let result = add(1, 1);
    assert_eq!(result, 2);
  }

  #[test]
  fn t_division() {
    assert_eq!(division(10.0, 2.0), Ok(5.0));
    assert_eq!(division(10.0, 0.0), Err("Division by zero is not allowed"));
  }

  #[test]
  fn t_fibonacci() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
  }

  #[test]
  fn t_is_palindrome() {
    assert!(!is_palindrome("william"));
    assert!(is_palindrome("ovo"));
    assert!(is_palindrome(""));
    assert!(!is_palindrome("rust"));
  }
}
