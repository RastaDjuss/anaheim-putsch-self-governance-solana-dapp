// FILE: anchor/programs/anaheim/src/contexts/initialize.rs
use anchor_lang::prelude::*;
use crate::state::AnaheimAccount; // Correct import from the state module

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = payer, space = AnaheimAccount::SIZE)]
  pub anaheim: Account<'info, AnaheimAccount>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}