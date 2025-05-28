#[cfg(test)]
mod tests {
  use super::*;
  use crate::error_code::{ErrorCode};
  use crate::validate_username;
  #[test]
  fn test_valid_username() {
    assert!(validate_username("Alice").is_ok());
  }

  #[test]
  fn test_empty_username() {
    assert_eq!(
      validate_username("   ").unwrap_err(),
      InvalidUsername.into()
    );
  }

  fn validate_username(p0: &str) -> _ {
    todo!()
  }

  #[test]
  fn test_too_long_username() {
    let long = "a".repeat(40);
    assert_eq!(
      validate_username(&long).unwrap_err(),
      ErrorCode::UsernameTooLong.into()
    );
  }
}
