use anchor_lang::prelude::*;

use crate::contexts::create_post::CreatePost;
use crate::error::ErrorCode;

pub fn handler(ctx: Context<CreatePost>, content: String) -> Result<()> {
  let trimmed = content.trim();

  if trimmed.is_empty() {
    return err!(ErrorCode::InvalidContent);
  }
  if trimmed.len() > 280 {
    return err!(ErrorCode::ContentTooLong);
  }

  let post_account = &mut ctx.accounts.post_account;

  post_account.content = [0u8; 280];
  let bytes = trimmed.as_bytes();
  let len = bytes.len().min(280);
  post_account.content[..len].copy_from_slice(&bytes[..len]);

  post_account.author = *ctx.accounts.user.key;
  post_account.created_at = Clock::get()?.unix_timestamp;

  msg!("Post created by {:?}", post_account.author);
  Ok(())
}
