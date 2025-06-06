use anchor_lang::prelude::*;

use crate::contexts::create_post::CreatePost;


pub fn handler(_ctx: Context<CreatePost>) -> Result<()> {
  // handler logic (exemple)
  Ok(())
}
