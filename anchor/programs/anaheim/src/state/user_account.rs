// ===================== state/user_account.rs =====================
use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
  pub username: [u8; 32],
  pub authority: Pubkey,
  pub timestamp: i64,
  pub bump: u8,
}

impl UserAccount {
  pub const SIZE: usize = 32 + 32 + 8 + 1; // username + authority + timestamp + bump
}

