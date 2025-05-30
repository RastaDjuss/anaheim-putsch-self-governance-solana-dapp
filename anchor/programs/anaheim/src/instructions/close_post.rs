// === instructions/close_post.rs ===
use anchor_lang::prelude::*;
use crate::state::PostAccount;

#[derive(Accounts)]
pub struct ClosePost<'info> {
  #[account(mut, close = user)]
  pub post: Account<'info, PostAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}
