// FILE: anchor/programs/anaheim/src/state/user_account.rs
use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
  pub username: String,
  pub authority: Pubkey,
  pub bump: u8,
  pub timestamp: i64,
}

impl UserAccount {
  pub const SIZE: usize = 8 + 4 + 32 + 1 + 8;
}
