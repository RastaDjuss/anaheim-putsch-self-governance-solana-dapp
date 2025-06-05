// ===================== state/user_account.rs =====================
use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
  pub username: [u8; 32],
  pub authority: Pubkey,
  pub timestamp: i64,
}

impl UserAccount {
  pub const SIZE: usize = 32 + 32 + 8; // username + pubkey + timestamp
}
