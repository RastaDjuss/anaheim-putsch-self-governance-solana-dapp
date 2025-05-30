use anchor_lang::prelude::*;
use crate::state::AnaheimAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = AnaheimAccount::SIZE)]
  pub anaheim: Account<'info, AnaheimAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
  let account = &mut ctx.accounts.anaheim;
  account.count = 0;
  Ok(())
}
