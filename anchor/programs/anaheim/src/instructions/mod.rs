// === instructions/mod.rs ===
pub mod create_post;
pub mod close_post;
pub mod initialize;

// === instructions/create_post.rs ===
use anchor_lang::prelude::*;
use crate::state::PostAccount;

#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(init, payer = user, space = PostAccount::LEN)]
  pub post: Account<'info, PostAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}
