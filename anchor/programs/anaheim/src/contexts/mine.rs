use crate::state::post_account::PostAccount;
// File: contexts/mine.rs
// --------------------------
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Mine<'info> {
    #[account(mut)]
    pub post: Account<'info, PostAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
