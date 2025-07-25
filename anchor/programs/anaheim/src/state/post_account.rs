// FILE: anchor/programs/anaheim/src/state/post_account.rs
use anchor_lang::prelude::*;
use crate::constants::MAX_CONTENT_LENGTH;

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
  pub vote_count: i64, // <-- ADD THIS LINE
}
impl PostAccount {
  // Update the size to include the new field
  pub const SIZE: usize = 8 + (4 + MAX_CONTENT_LENGTH) + 32 + 8 + 8;
}