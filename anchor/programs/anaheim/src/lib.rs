use anchor_lang::prelude::*;
pub mod contexts;
pub mod handlers;
pub mod instructions;
pub mod state;
pub mod error;
pub mod close;
pub mod constants;
pub mod post;
pub mod validate_post_content;
pub mod validate_username;
use crate::contexts::Update;
pub use error::ErrorCode;
use crate::handlers::increment_handler;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod anaheim {
  use super::*;

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    increment_handler::handle_increment(ctx)
  }
}
