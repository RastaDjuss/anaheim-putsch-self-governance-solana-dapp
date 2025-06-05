#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::prelude::program;
pub mod constants;
pub mod error;
pub mod state;
pub mod instructions;
pub mod handlers;
pub mod close;
pub mod utils;
pub mod contexts;
pub use crate::utils::validation::validate_username;
pub use crate::utils::validation::validate_post_content;

use crate::instructions::use_anaheim::UseAnaheim;
use crate::instructions::vote_post::VotePost;
use crate::state::state::PostAccount;
use crate::contexts::initialize::Initialize;
use crate::contexts::create_post::CreatePost;
use crate::contexts::create_user::CreateUser;
use crate::handlers::create_user;
use crate::contexts::update::Update;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod anaheim {
  use crate::handlers::{increment_handler, set};
  use crate::instructions::{use_anaheim, vote_post};
  use super::*;

  pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
    create_user::handler(ctx, username)
  }

  pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
    handlers::create_post::handler(ctx, content)
  }


  pub fn vote_post(ctx: Context<VotePost>, upvote: bool) -> Result<()> {
    vote_post::handler(ctx, upvote)
  }

  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    handlers::initialize_handler(ctx)
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    increment_handler::handler(ctx)
  }

  pub fn use_anaheim(ctx: Context<UseAnaheim>) -> Result<()> {
    use_anaheim::handler(ctx)
  }

  pub fn handle_set(ctx: Context<Update>, _value: u8) -> Result<()> {
    set::handler(ctx, _value)
  }
  pub fn create_post_handler(ctx: Context<CreatePost>, content: String) -> Result<()> {
    handlers::create_post::handler(ctx, content)
  }

  pub fn test_import(_ctx: Context<Dummy>) -> Result<()> {
    msg!("Import works.");
    Ok(())
  }

  pub fn test(_ctx: Context<Test>) -> Result<()> {
    msg!("PostAccount size is {}", PostAccount::SIZE);
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Dummy {}

#[derive(Accounts)]
pub struct Test {}
