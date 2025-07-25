// ===================== handlers/shared.rs =====================
use anchor_lang::prelude::*;

use crate::contexts::update::UpdatePost;
use crate::contexts::create_user::CreateUser;
use crate::contexts::create_post::CreatePost;
use crate::error::ErrorCode;
use crate::instructions::initialize::Initialize;

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
  let account = &mut ctx.accounts.anaheim;
  account.count = 0;
  account.value = 0;
  Ok(())
}

pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
  if content.len() > 280 {
    return Err(ErrorCode::InvalidContent.into());
  }
  let post_account = &mut ctx.accounts.post_account;
  post_account.content = string_to_fixed(&content);
  post_account.author = *ctx.accounts.user.key;
  post_account.created_at = Clock::get()?.unix_timestamp;
  msg!("Post created: {}", content);
  Ok(())
}

fn string_to_fixed(p0: &String) -> [u8; 280] {
  todo!()
}

pub fn set_value(ctx: Context<UpdatePost>, value: u8) -> Result<()> {
  let anaheim_account = &mut ctx.accounts.anaheim;
  anaheim_account.value = value;
  Ok(())
}

pub fn increment(ctx: Context<UpdatePost>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count = anaheim.count.checked_add(1).ok_or(ErrorCode::Overflow)?;
  Ok(())
}

pub fn decrement(ctx: Context<UpdatePost>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count = anaheim.count.checked_sub(1).ok_or(ErrorCode::Underflow)?;
  Ok(())
}

pub fn handle_increment(ctx: Context<UpdatePost>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.value = anaheim.value.checked_add(1).unwrap_or(anaheim.value);
  Ok(())
}
