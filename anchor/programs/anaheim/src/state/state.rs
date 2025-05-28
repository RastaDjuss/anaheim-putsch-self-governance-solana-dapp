use anchor_lang::prelude::*;

#[account]
pub struct PostAccount {
  pub owner: Pubkey,
  pub content: String,
}

#[account]
#[derive(Debug)]
pub struct UserAccount {
  pub authority: Pubkey,
  pub username: String,
}

impl UserAccount {
  pub const SIZE: usize = 8 + 32 + 4 + 32; // discriminator + Pubkey + string prefix + 32 max chars
}
