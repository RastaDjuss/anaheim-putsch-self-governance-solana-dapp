use anchor_lang::prelude::*;

#[account]
pub struct Anaheim {
  pub authority: Pubkey,
  pub value: u64,
  pub count: u64,
  pub counter: u8, // <-- il te faut CE champ si tu veux le manipuler
}

impl Anaheim {
  pub const INIT_SPACE: usize = 1; // u8 = 1 byte
}
#[account]
pub struct AnaheimAccount {
  pub count: u64,
}

