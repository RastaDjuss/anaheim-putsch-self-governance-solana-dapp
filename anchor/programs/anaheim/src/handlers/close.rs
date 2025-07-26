// programs/anaheim/src/handlers/close.rs
use anchor_lang::prelude::*;
use crate::close::close_account::CloseAccount;
pub use crate::close::close_post::ClosePost;


pub fn close_post_account(_ctx: Context<CloseAccount>) -> Result<()> {
    // Anchor va automatiquement close le compte car tu utilises `#[account(close = user)]`
    Ok(())
}
