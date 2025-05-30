use anchor_lang::prelude::*;

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub created_at: i64,
}

impl PostAccount {
  // Max content length aligned with program constant
  pub const MAX_CONTENT_LENGTH: usize = 280;
  pub const SIZE: usize = 8 + 4 + Self::MAX_CONTENT_LENGTH + 32 + 8;
}
