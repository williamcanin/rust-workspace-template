/// Gets the path of the current user's root directory, on Windows or Linux, and concatenates it
/// with others through a parameter.
#[macro_export]
macro_rules! userprofile {
  ($name:expr) => {
    if cfg!(windows) {
      std::path::Path::new(&std::env::var("USERPROFILE").unwrap()).join($name.replace("/", r"\"))
    } else {
      std::path::Path::new(&std::env::var("HOME").unwrap()).join($name.replace("/", "/"))
    }
  };
}
