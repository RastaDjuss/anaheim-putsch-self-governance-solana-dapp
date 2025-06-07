// ===================== contexts/update.rs =====================
use anchor_lang::prelude::*;
use crate::state::AnaheimAccount;

#[derive(Accounts)]
pub struct UpdatePost<'info> {
  #[account(mut)]
  pub anaheim: Account<'info, AnaheimAccount>,
}
