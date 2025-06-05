use anchor_lang::prelude::*;

pub const MAX_CONTENT_LENGTH: usize = 256;
pub const MAX_USERNAME_LENGTH: usize = 32;

#[account]
pub struct UserAccount {
  pub name: String,
  pub user_authority: Pubkey,
}

impl UserAccount {
  pub const SIZE: usize = 8 + 4 + MAX_USERNAME_LENGTH + 32;
}

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
  pub value: u8,
}

impl PostAccount {
  pub const SIZE: usize = 8 + 4 + MAX_CONTENT_LENGTH + 32 + 8 + 1;
}

#[account]
pub struct Anaheim {
  pub authority: Pubkey,
  pub value: u8,
  pub count: u64,
}

impl Anaheim {
  pub const SIZE: usize = 8 + 32 + 1 + 8 + 1;
}

