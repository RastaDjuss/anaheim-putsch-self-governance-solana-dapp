
// === FILE: programs/anaheim/src/constants.rs ===
pub const MAX_USERNAME_LENGTH: usize = 32;
pub const MAX_MESSAGE_LENGTH: usize = 256;
// Tailor this value as needed for your app's maximum allowed content length
pub const MAX_CONTENT_LENGTH: usize = 280; // or whatever max you want (like Twitter or Reddit)


// === FILE: programs/anaheim/src/error.rs ===
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Invalid username.")]
  InvalidUsername,

  #[msg("Content exceeds max allowable length.")]
  ContentTooLong,

  #[msg("Invalid content.")]
  InvalidContent,
}
