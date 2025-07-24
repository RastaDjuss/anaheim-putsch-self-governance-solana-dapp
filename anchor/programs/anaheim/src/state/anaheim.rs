// anchor/programs/anaheim/src/state/anaheim.rs

use anchor_lang::prelude::*;

// ✅ Ceci suffit
#[account]
pub struct Anaheim {
  pub authority: Pubkey,
  pub count: u64,
  pub value: u8,
}

impl Anaheim {
  pub const SIZE: usize = 8 + 32 + 8 + 1;
}
