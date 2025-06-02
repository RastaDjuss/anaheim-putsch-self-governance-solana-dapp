pub mod initialize_handler;
pub mod handler;
pub mod decrement_handler;
pub(crate) mod increment;
pub mod increment_handler;

use crate::state::post_account::PostAccount;
use anchor_lang::prelude::*;
use crate::Update;
#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(init, payer = user, space = 8 + 280)]
  pub post: Account<'info, PostAccount>,

  pub system_program: Program<'info, System>,
}
pub(crate) fn set(ctx: Context<Update>, value: u8) -> Result<()> {
  let anaheim_account = &mut ctx.accounts.anaheim;
  anaheim_account.value = value;  // ASSIGNATION DIRECTE SUR LE CHAMP
  Ok(())
}


