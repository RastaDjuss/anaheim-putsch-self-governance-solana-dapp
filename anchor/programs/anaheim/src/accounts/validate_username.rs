use anchor_lang::prelude::*;
use crate::error::ErrorCode;

pub fn validate_username(username: &str) -> Result<()> {
  let trimmed = username.trim();
  if trimmed.is_empty() {
    return Err(ErrorCode::InvalidUsername.into());
  }
  if trimmed.len() > 32 {
    return Err(ErrorCode::UsernameTooLong.into());
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_username() {
    assert!(validate_username("Alice").is_ok());
  }

  #[test]
  fn test_empty_username() {
    assert_eq!(
      validate_username("   ").unwrap_err(),
      ErrorCode::InvalidUsername.into()
    );
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
