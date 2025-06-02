use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod close;
pub mod constants;
pub mod handlers;
pub mod post;
pub mod validate_post_content;
pub mod validate_username;
pub mod contexts;
use crate::contexts::Update;
use crate::handlers::{increment_handler::handle_increment, decrement_handler::handle_decrement};
pub use error::ErrorCode;
use crate::program::Anaheim;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod anaheim {
  use super::*;

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    handle_increment(ctx)
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    handle_decrement(ctx)
  }
  pub fn handle_increment(ctx: Context<Update>) -> Result<()> {
    let anaheim = &mut ctx.accounts.anaheim;
    anaheim.counter = anaheim.counter.saturating_add(1);
    Ok(())
  }
}

