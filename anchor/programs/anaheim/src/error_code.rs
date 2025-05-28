use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("The username exceeds the maximum length of 32 characters.")]
  UsernameTooLong,
}
