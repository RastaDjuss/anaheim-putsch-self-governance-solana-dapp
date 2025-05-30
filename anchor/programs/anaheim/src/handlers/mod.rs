// === handlers/mod.rs ===
pub mod create_post;
pub mod close_post;
pub use close_post::*;

// === handlers/create_post.rs ===
use anchor_lang::prelude::*;
use crate::instructions::CreatePost;

pub fn handle_create_post(_ctx: Context<CreatePost>) -> Result<()> {
  // Post creation logic here
  Ok(())
}
