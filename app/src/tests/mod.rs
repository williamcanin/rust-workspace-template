#[cfg(test)]
mod tests {
  use crate::utils::check::has_connection;

  #[test]
  fn t_has_connection() {
    match  has_connection() {
      true => assert!(true),
      false => {
        println!(">>>> You are NOT connected to the internet <<<<");
        assert!(false)
      }
    }
  }
}
