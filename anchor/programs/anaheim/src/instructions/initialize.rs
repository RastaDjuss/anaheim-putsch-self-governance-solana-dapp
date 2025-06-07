// === instructions/initialize.rs ou contexts/initialize.rs ===
use anchor_lang::prelude::*;

use crate::state::AnaheimAccount; // ou le chemin correct vers le compte

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + AnaheimAccount::SIZE
  )]
  pub anaheim: Account<'info, AnaheimAccount>,  // <-- ici la bonne structure

  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>,
}
