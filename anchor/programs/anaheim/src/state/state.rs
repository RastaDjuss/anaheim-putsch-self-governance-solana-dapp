use anchor_lang::prelude::*;
use anchor_lang::system_program::ID;
use crate::constants::MAX_CONTENT_LENGTH;
pub const MAX_CONTENT_LENGTH: usize = 256;
pub const MAX_USERNAME_LENGTH: usize = 32;

#[account]
pub struct UserAccount {
  pub name: String,
  pub user_authority: Pubkey,
}

impl UserAccount {
  pub const SIZE: usize = 8 + 4 + MAX_USERNAME_LENGTH + 32;
  // 8 = discriminator, 4 = string prefix length, MAX_USERNAME_LENGTH = max string length, 32 = pubkey
}

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
  pub value: u8,
}

impl PostAccount {
  // discriminator (8) + string prefix (4) + max content (256) + pubkey (32) + timestamp (8)
  pub const SIZE: usize = 8 + 4 + MAX_CONTENT_LENGTH + 32 + 8;
}
#[account]
pub struct Anaheim {
  pub authority: Pubkey,
  pub value: u8,
  pub count: u64,
  pub counter: u8, // <-- il te faut CE champ si tu veux le manipuler
}

impl Anaheim {
  pub const INIT_SPACE: usize = 1;
}

#[account]
pub struct AnaheimAccount {
  pub count: u64,
}

