// === state/anaheim_account.rs ===
use anchor_lang::prelude::*;

#[account]
pub struct AnaheimAccount {
  pub count: u64,
}

impl AnaheimAccount {
  pub const SIZE: usize = 8 + 8; // discriminator + count
}
