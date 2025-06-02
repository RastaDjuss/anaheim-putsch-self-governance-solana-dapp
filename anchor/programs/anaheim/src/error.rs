
// === FILE: programs/anaheim/src/error.rs ===
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Invalid username.")]
  InvalidUsername,

  #[msg("Username Too Long...")]
  UsernameTooLong,

  #[msg("Content exceeds max allowable length.")]
  ContentTooLong,

  #[msg("Invalid content.")]
  InvalidContent,

  #[msg("Already voted.")]
  AlreadyVoted,

  #[msg("Overflow occurred.")]
  Overflow,

  #[msg("Underflow occurred.")]
  Underflow,
}
