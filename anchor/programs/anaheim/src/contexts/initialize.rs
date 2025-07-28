// FILE: anchor/programs/anaheim/src/contexts/initialize.rs
use anchor_lang::prelude::*;
use crate::state::anaheim_account::AnaheimAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = payer, space = AnaheimAccount::SIZE, seeds = [b"anaheim", payer.key().as_ref()], bump)]
  pub anaheim: Account<'info, AnaheimAccount>,

  #[account(mut)]
  pub payer: Signer<'info>,

  pub system_program: Program<'info, System>,
}
