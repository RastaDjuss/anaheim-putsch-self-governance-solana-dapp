// === state/post_account.rs ===
use anchor_lang::prelude::*;

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub created_at: i64,
}

impl PostAccount {
  pub const LEN: usize = 8 + 4 + 280 + 32 + 8;
}
