use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(mut)]
  pub author: Signer<'info>,
  // Add other accounts here, like:
  // #[account(init, payer = author, space = 8 + Post::LEN)]
  // pub post_account: Account<'info, Post>,
  pub system_program: Program<'info, System>,
}
