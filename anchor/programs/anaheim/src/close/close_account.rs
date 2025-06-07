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
  // Here the #[account(close = user)] attribute will automatically
  // transfer lamports from the closed post_account to the user.

  msg!("Closing post account: {}", ctx.accounts.post_account.content);

  // No need to manually clear fields; the account data will be deallocated.
  Ok(())
}
