use anchor_lang::prelude::*;
use crate::instructions::create_post::CreatePost;
use crate::instructions::close_post::{ClosePost, handle_close_post};
use crate::error::ErrorCode;

const MAX_MESSAGE_LENGTH: usize = 256;

#[program]
pub mod anaheim {
  use super::*;

  pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
    if content.len() > MAX_MESSAGE_LENGTH {
      return err!(ErrorCode::ContentTooLong);
    }

    let post = &mut ctx.accounts.post_account;
    post.content = content.clone();
    post.author = *ctx.accounts.user.key;
    post.created_at = Clock::get()?.unix_timestamp;

    msg!("Post created successfully: {}", post.content);
    Ok(())
  }

  pub fn close_post(ctx: Context<ClosePost>) -> Result<()> {
    handle_close_post(ctx)
  }
}
