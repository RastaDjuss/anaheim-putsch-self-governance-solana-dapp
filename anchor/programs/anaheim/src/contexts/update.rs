// ===================== contexts/update.rs =====================
use anchor_lang::prelude::*;

use crate::state::Anaheim;

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub anaheim: Account<'info, Anaheim>,
}
