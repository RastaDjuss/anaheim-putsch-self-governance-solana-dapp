use anchor_lang::prelude::*;

use crate::handlers::post_account::PostAccount;
use crate::constants::MAX_CONTENT_LENGTH;
#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + 4 + MAX_CONTENT_LENGTH + 64 + 8 + 4
  )]
  pub post: Account<'info, PostAccount>,

  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>,
}
pub fn handler(_ctx: Context<CreatePost>) -> Result<()> {
  // handler logic (exemple)
  Ok(())
}
