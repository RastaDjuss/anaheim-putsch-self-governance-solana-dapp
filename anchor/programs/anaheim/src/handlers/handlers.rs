// === FILE: programs/anaheim/src/handlers/handlers.rs ===
use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::instructions::{create_user::CreateUser, create_post::CreatePost};

pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
  if username.len() > 32 {
    return Err(ErrorCode::InvalidContent.into());
  }
  let user_account = &mut ctx.accounts.user_account;
  user_account.username = username.clone();
  user_account.authority = *ctx.accounts.user.key;
  user_account.timestamp = Clock::get()?.unix_timestamp;

  msg!("User created: {}", user_account.username);
  Ok(())
}

pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
  if content.len() > 280 {
    return Err(ErrorCode::InvalidContent.into());
  }
  let post_account = &mut ctx.accounts.post_account;
  post_account.content = content.clone();
  post_account.author = *ctx.accounts.user.key;
  post_account.created_at = Clock::get()?.unix_timestamp;

  msg!("Post created: {}", post_account.content);
  Ok(())
}
