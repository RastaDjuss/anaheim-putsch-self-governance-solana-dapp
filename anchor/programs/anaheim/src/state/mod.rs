use anchor_lang::AnchorDeserialize;
pub mod anaheim_account;
pub mod post_account;
pub mod user_account;
use anchor_lang::prelude::Pubkey;
use anchor_lang::Discriminator;
use anchor_lang::account;
use anchor_lang::AnchorSerialize;
use anchor_lang::prelude::borsh;
pub use anaheim_account::Anaheim;

#[account]
pub struct AnaheimAccount {
  pub count: u64,
}

impl AnaheimAccount {
  pub const SIZE: usize = 8 + 8; // 8 bytes discriminator + 8 bytes u64
}
