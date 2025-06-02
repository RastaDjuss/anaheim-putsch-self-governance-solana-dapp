use anchor_lang::prelude::*;
use crate::state::anaheim_account::Anaheim;

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(
        init,
        payer = user,
        space = 8 + Anaheim::INIT_SPACE,
  )]
  pub anaheim: Account<'info, Anaheim>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.counter = 0;
  msg!("Anaheim initialized");
  Ok(())
}
