// File: programs/anaheim/src/lib.rs

#![allow(deprecated)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// ─── BASE ─────────────────────────────
use anchor_lang::prelude::*;

// ─── MODULES ──────────────────────────
pub mod close;
pub mod constants;
pub mod contexts;
pub mod error;
pub mod handlers;
pub mod instructions;
pub mod post;
pub mod state;
pub mod test_post_account;
pub mod utils;
pub mod validate_post_content;

// ─── CONTEXT SHORTCUTS ────────────────
pub use contexts::{
  initialize,
  create_user,
  create_post,
  vote_post,
  mine,
  update,
};
pub use instructions::use_anaheim;

declare_id!("CKqr1tXUJxTkkBdkbpWNWEEyDBkrfus5JXKgZJcXJZBf");

// ─── PROGRAM ──────────────────────────
#[program]
pub mod anaheim {
  use super::*;

  pub fn initialize(ctx: Context<initialize::Initialize>) -> Result<()> {
    handlers::initialize_handler::initialize_handler(ctx)
  }

  pub fn create_user(ctx: Context<create_user::CreateUser>, username: String) -> Result<()> {
    handlers::create_user::handle_create_user(ctx, username)
  }

  pub fn create_post(ctx: Context<create_post::CreatePost>, _content: String) -> Result<()> {
    handlers::handle_create_post::handle_create_post(ctx)
  }

  pub fn vote_post(ctx: Context<vote_post::VotePost>, bump: u8, upvote: bool) -> Result<()> {
    instructions::vote_post::handler(ctx, bump, upvote)
  }

  pub fn mine(ctx: Context<mine::Mine>) -> Result<()> {
    handlers::handler_mine::handle_mine(ctx)
  }

  pub fn update_post(ctx: Context<update_post::UpdatePost>, new_content: String) -> Result<()> {
    handlers::update_post::handle_update_post(ctx, new_content)
  }

  pub fn increment(ctx: Context<use_anaheim::UseAnaheim>) -> Result<()> {
    handlers::handle_increment::handle_increment(ctx)
  }

  pub fn decrement(ctx: Context<update::UpdatePost>) -> Result<()> {
    ctx.accounts.anaheim.count = ctx.accounts.anaheim.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment_by_one(ctx: Context<update::UpdatePost>) -> Result<()> {
    ctx.accounts.anaheim.count = ctx.accounts.anaheim.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn set(ctx: Context<use_anaheim::UseAnaheim>, value: u64) -> Result<()> {
    ctx.accounts.anaheim.count = value;
    Ok(())
  }

  pub fn close(ctx: Context<close::CloseAnaheim>) -> Result<()> {
    Ok(())
  }
}


