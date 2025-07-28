// FILE: anchor/programs/anaheim/src/state/post_account.rs
use anchor_lang::prelude::*;

pub const MAX_CONTENT_LENGTH: usize = 280;

#[account]
pub struct PostAccount {
  pub content: [u8; MAX_CONTENT_LENGTH],
  pub author: Pubkey,
  pub created_at: i64,
  pub vote_count: u64,
}


impl PostAccount {
  pub const SIZE: usize = 8 + MAX_CONTENT_LENGTH + 32 + 8 + 8;
}