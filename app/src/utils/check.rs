pub fn has_connection() -> bool {
  use std::net::TcpStream;
  match TcpStream::connect("google.com:80") {
    Ok(_) => true,
    Err(_) => false,
  }
}
