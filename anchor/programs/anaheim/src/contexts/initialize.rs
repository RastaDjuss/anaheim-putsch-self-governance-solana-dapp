use anchor_lang::prelude::*;

use crate::state::anaheim::Anaheim;

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(
        init,
        payer = user,
        space = 8 + Anaheim::SIZE
  )]
  pub anaheim: Account<'info, Anaheim>,

  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>,
}
