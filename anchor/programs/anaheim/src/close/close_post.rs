use anchor_lang::prelude::*;
use crate::state::post_account::PostAccount;

#[derive(Accounts)]
pub struct ClosePost<'info> {
  #[account(mut, close = user)]
  pub post_account: Account<'info, PostAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

pub fn close_post(ctx: Context<ClosePost>) -> Result<()> {
  msg!("Closing post account: {}", ctx.accounts.post_account.content);
  Ok(())
}
