// instructions/create_post.rs
use anchor_lang::prelude::*;
use crate::constants::MAX_POST_SIZE;
use crate::state::post_account::PostAccount;
use solana_program::sysvar::clock;



#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
        init,
        payer = user,
        space = 8 + MAX_POST_SIZE,
        seeds = [b"post", user.key().as_ref(), clock.unix_timestamp.to_le_bytes().as_ref()],
        bump
  )]
  pub post: Account<'info, PostAccount>,

  pub system_program: Program<'info, System>,
  #[account(address = clock::ID)]
  pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<CreatePost>, content: String) -> Result<()> {
  let post = &mut ctx.accounts.post;
  post.author = ctx.accounts.user.key();
  post.timestamp = ctx.accounts.clock.unix_timestamp;
  post.content = content;
  post.vote_count = 0;
  Ok(())
}
