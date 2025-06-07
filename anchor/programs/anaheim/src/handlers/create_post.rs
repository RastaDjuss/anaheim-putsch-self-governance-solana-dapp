use anchor_lang::prelude::*;

use crate::post::Post;

#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(init, payer = user, space = Post::LEN)]
  pub post_account: Account<'info, Post>,

  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>,
}
