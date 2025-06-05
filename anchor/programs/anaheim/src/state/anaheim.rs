use anchor_lang::prelude::*;

#[account]
pub struct Anaheim {
  pub authority: Pubkey,
  pub value: u8,
  pub count: u64,
}

impl Anaheim {
  pub const SIZE: usize = 8 + // discriminator
    32 + // authority
    1 +  // value
    8;   // count
}

