use anchor_lang::prelude::*;

// ===================== handlers/mod.rs =====================
mod shared;
pub mod initialize_handler;
pub mod decrement_handler;
pub mod increment_handler;
pub mod handle_increment;
pub mod set;
pub mod create_post;
pub mod create_user;
mod handle_create_user;

pub use create_user::*;

pub use initialize_handler::initialize_handler;
use crate::contexts::update::Update;

pub(crate) fn set(_ctx: Context<Update>) -> Result<()> {
  Ok(())
}


