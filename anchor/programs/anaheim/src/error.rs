use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("The provided content is invalid or empty.")]
  InvalidContent,

  #[msg("The content length exceeds the maximum allowed.")]
  ContentTooLong,

  #[msg("The username length exceeds the maximum allowed.")]
  UsernameTooLong,
  #[msg("Username is invalid.")]
  InvalidUsername,
  // Add other errors as needed
}
