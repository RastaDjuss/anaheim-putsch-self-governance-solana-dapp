// FILE: anchor/programs/anaheim/src/state/user_account.rs
use anchor_lang::prelude::*;
use crate::constants::MAX_USERNAME_LENGTH;

#[account]
pub struct UserAccount {
  pub name: String,
  pub user_authority: Pubkey,
}
impl UserAccount {
  pub const SIZE: usize = 8 + (4 + MAX_USERNAME_LENGTH) + 32;
}