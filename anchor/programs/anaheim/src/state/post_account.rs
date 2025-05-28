use anchor_lang::prelude::*;

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
}

impl PostAccount {
  pub const SIZE: usize = 8 + (4 + 256) + 32 + 8; // Update if max message length changes.
}
