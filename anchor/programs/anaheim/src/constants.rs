use anchor_lang::{err, error_code};
pub(crate) use crate::constants::MAX_MESSAGE_LENGTH as MAX_CONTENT_LENGTH;



#[error_code]
pub enum ErrorCode {
  #[msg("Content exceeds max allowable length.")]
  ContentTooLong,
  #[msg("Username exceeds max allowable length.")]
  UsernameTooLong,
  #[msg("Content is invalid (empty or whitespace only).")]
  InvalidContent,
}

pub const MAX_MESSAGE_LENGTH: usize = 256;
pub const MAX_USERNAME_LENGTH: usize = 32;

pub fn validate_content(content: &str) -> Result<(), ErrorCode> {
    let trimmed_content = content.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    if trimmed_content.is_empty() {
        return err!(ErrorCode::InvalidContent);
    }
    Ok(())
}
pub fn validate_username(username: &str) -> Result<(), ErrorCode> {
  let trimmed_username = username.trim();
  if trimmed_username.is_empty() {
    return Err(ErrorCode::InvalidContent); // Ajout d'une validation contre les chaînes vides.
  }
  if trimmed_username.len() > MAX_USERNAME_LENGTH {
    return Err(ErrorCode::UsernameTooLong.into());
  }
  Ok(())
}


