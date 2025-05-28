pub const MAX_MESSAGE_LENGTH: usize = 256;

pub fn validate_post_content(content: &str) -> Result<(), &'static str> {
  if content.trim().is_empty() {
    return Err("Content is empty.");
  }
  if content.len() > MAX_MESSAGE_LENGTH {
    return Err("Content exceeds max allowable length.");
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_content() {
    let result = validate_post_content("");
    assert_eq!(result.unwrap_err(), "Content is empty.");
  }

  #[test]
  fn test_content_too_long() {
    let result = validate_post_content(&"a".repeat(MAX_MESSAGE_LENGTH + 1));
    assert_eq!(result.unwrap_err(), "Content exceeds max allowable length.");
  }

  #[test]
  fn test_valid_content() {
    let result = validate_post_content("Valid post");
    assert!(result.is_ok());
  }

  #[test]
  fn test_content_max_length() {
    let result = validate_post_content(&"a".repeat(MAX_MESSAGE_LENGTH));
    assert!(result.is_ok());
  }

  #[test]
  fn test_special_chars() {
    let result = validate_post_content("🚀✨💡");
    assert!(result.is_ok());
  }
}
