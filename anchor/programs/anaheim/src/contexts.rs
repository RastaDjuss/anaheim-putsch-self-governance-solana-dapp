use anchor_lang::Accounts;
use anchor_lang::prelude::{Account, Signer};
use crate::state::anaheim_account::Anaheim;
use anchor_lang::prelude::AccountInfo;

// contexts.rs
#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub anaheim: Account<'info, Anaheim>,
  pub authority: Signer<'info>,
}
