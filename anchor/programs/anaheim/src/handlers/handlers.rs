// === FILE: programs/anaheim/src/handlers/handlers.rs ===
use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::instructions::{create_user::CreateUser, create_post::CreatePost};
use crate::anaheim_init::Initialize;
use crate::state::Anaheim;
use crate::contexts::Update;
use state::Anaheim;
use crate::contexts::Update;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
  let account = &mut ctx.accounts.anaheim;
  account.count = 0;
  Ok(())
}
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count = 0;
  Ok(())
}
pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
  let anaheim_account = &mut ctx.accounts.anaheim;
  anaheim_account.value = value; // ✅ accède bien au champ `value`
  Ok(())
}

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
#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub anaheim: Account<'info, AnaheimAccount>,
}

pub fn increment(ctx: Context<Update>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count = anaheim.count.checked_add(1).ok_or(ErrorCode::Overflow)?;
  Ok(())
}

pub fn decrement(ctx: Context<Update>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count = anaheim.count.checked_sub(1).ok_or(ErrorCode::Underflow)?;
  Ok(())
}
pub fn handle_increment(ctx: Context<Update>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.value = anaheim.value.checked_add(1).unwrap_or(anaheim.value);
  Ok(())
}
