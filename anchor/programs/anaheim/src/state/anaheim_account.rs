use anchor_lang::prelude::*;

#[account]
pub struct AnaheimAccount {
  pub authority: Pubkey,
  pub value: u8,
  pub count: u64,
}

impl AnaheimAccount {
  pub const SIZE: usize = 8 + 32 + 1 + 8;
}
