use anchor_lang::prelude::*;
use crate::state::AnaheimAccount;

#[derive(Accounts)]
pub struct UseAnaheim<'info> {
  #[account(mut)]
  pub anaheim: Account<'info, AnaheimAccount>,
}

pub fn handler(_ctx: Context<UseAnaheim>) -> Result<()> {
  Ok(())
}
