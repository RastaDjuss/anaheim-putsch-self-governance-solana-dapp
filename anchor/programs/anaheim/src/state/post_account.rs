// FILE: anchor/programs/anaheim/src/state/post_account.rs
use anchor_lang::prelude::*;
use crate::constants::MAX_CONTENT_LENGTH;

#[account]
pub struct PostAccount {
  pub content: [u8; MAX_CONTENT_LENGTH], // Using fixed-size array
  pub author: Pubkey,
  pub created_at: i64,
}

impl PostAccount {
  pub const SIZE: usize = 8 + MAX_CONTENT_LENGTH + 32 + 8;
}