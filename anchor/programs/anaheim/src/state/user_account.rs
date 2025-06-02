use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
  pub username:  String,
  pub authority: Pubkey,
  pub timestamp: i64,
}

impl UserAccount {
  pub const MAX_USERNAME_LENGTH: usize = 32;
  pub const SIZE: usize =
    8 + 4 + Self::MAX_USERNAME_LENGTH + 32 + 8;
  // 8 discriminator + 4 string prefix + max username + 32 Pubkey + 8 timestamp
}
