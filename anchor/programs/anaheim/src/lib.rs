use anchor_lang::prelude::*;
use anchor_lang::system_program::ID;
pub mod constants;
pub mod error;
pub mod state;
pub mod handlers;
pub mod instructions;

pub use crate::error::ErrorCode;
pub use instructions::*; // <-- pulls in CreatePost, ClosePost, etc. from instructions/mod.rs
use handlers::{handle_create_post, handle_close_post};
#[program]
pub mod anaheim {
  use super::*;

  pub fn create_post(ctx: Context<CreatePost>) -> Result<()> {
    handle_create_post(ctx)
  }

  pub fn close_post(ctx: Context<ClosePost>) -> Result<()> {
    handle_close_post(ctx)
  }
}
