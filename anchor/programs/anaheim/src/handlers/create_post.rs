use anchor_lang::prelude::*;
use crate::instructions::CreatePost;

pub fn handle_create_post(_ctx: Context<CreatePost>) -> Result<()> {
    // Post logic
    Ok(())
}
