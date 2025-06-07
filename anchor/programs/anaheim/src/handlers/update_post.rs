use anchor_lang::prelude::*;

pub use crate::state::post_account::PostAccount;

#[derive(Accounts)]
pub struct UpdatePost<'info> {
  #[account(mut)]
  pub post_account: Account<'info, PostAccount>,
}
