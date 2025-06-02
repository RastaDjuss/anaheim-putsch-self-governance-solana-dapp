// programs/anaheim/src/instructions/update.rs

use anchor_lang::prelude::*;
use crate::state::anaheim_account::Anaheim;

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub anaheim: Account<'info, Anaheim>,
  pub authority: Signer<'info>,
}
pub fn increment(ctx: Context<Update>) -> Result<()> {
  ctx.accounts.anaheim.count = ctx.accounts.anaheim.count.checked_add(1).unwrap();
  Ok(())
}

pub fn decrement(ctx: Context<Update>) -> Result<()> {
  ctx.accounts.anaheim.count = ctx.accounts.anaheim.count.checked_sub(1)
    .ok_or(error!(crate::error::ErrorCode::Underflow))?;
  Ok(())
}

pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
  ctx.accounts.anaheim.count = value as u64;
  Ok(())
}

