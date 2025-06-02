use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseAnaheim<'info> {
  #[account(mut, close = payer)]
  pub anaheim: Account<'info, crate::state::anaheim_account::Anaheim>,

  #[account(mut)]
  pub payer: Signer<'info>,

  pub system_program: Program<'info, System>,
}
